//! Implementation pipeline — product implement FT-XXX (ADR-021)

use crate::config::ProductConfig;
use crate::context;
use crate::error::{ProductError, Result};
use crate::gap;
use crate::graph::KnowledgeGraph;
use std::path::Path;
use std::process::Command;

use super::verify::run_verify;

/// Run the 5-step implementation pipeline
pub fn run_implement(
    feature_id: &str,
    config: &ProductConfig,
    root: &Path,
    graph: &KnowledgeGraph,
    dry_run: bool,
    no_verify: bool,
    headless: bool,
) -> Result<()> {
    let feature = graph.features.get(feature_id).ok_or_else(|| {
        ProductError::NotFound(format!("feature {}", feature_id))
    })?;

    println!("product implement {}", feature_id);
    println!();

    // Step 0 — Preflight (domain + cross-cutting coverage)
    print!("  Step 0: Preflight... ");
    let preflight_result = crate::domains::preflight(graph, feature_id, &config.domains)?;
    if !preflight_result.is_clean {
        println!("BLOCKED");
        eprintln!();
        eprintln!("{}", crate::domains::render_preflight(&preflight_result));
        eprintln!("  resolve domain/cross-cutting gaps or acknowledge them before implementing.");
        return Err(ProductError::ConfigError("preflight not clean".to_string()));
    }
    println!("OK (all domains and cross-cutting ADRs covered)");

    // Step 1 — Gap gate
    print!("  Step 1: Gap gate... ");
    let baseline = gap::GapBaseline::load(&root.join(config.paths.gaps_resolved()));
    let mut all_findings = Vec::new();
    for adr_id in &feature.front.adrs {
        let findings = gap::check_adr(graph, adr_id, &baseline);
        all_findings.extend(findings);
    }
    let unsuppressed_high: Vec<_> = all_findings
        .iter()
        .filter(|f| f.severity == gap::GapSeverity::High && !f.suppressed)
        .collect();

    if !unsuppressed_high.is_empty() {
        println!("BLOCKED");
        eprintln!();
        eprintln!("error[E009]: implementation blocked by specification gaps");
        eprintln!("  feature: {} — {}", feature.front.id, feature.front.title);
        for g in &unsuppressed_high {
            eprintln!("  gap[{}]: {}", g.code, g.description);
        }
        eprintln!();
        eprintln!("  suppress gaps or add TCs before implementing.");
        return Err(ProductError::ConfigError("gap gate failed".to_string()));
    }
    println!("OK (no high-severity gaps)");

    // Step 2 — Drift check (advisory only)
    println!("  Step 2: Drift check... (advisory, skipped — no drift config)");

    // Step 3 — Context assembly
    print!("  Step 3: Context assembly... ");
    let bundle = context::bundle_feature(graph, feature_id, 2, true)
        .unwrap_or_default();

    // Build TC status table
    let mut tc_table = String::new();
    tc_table.push_str("| TC | Title | Type | Status |\n|---|---|---|---|\n");
    for tc_id in &feature.front.tests {
        if let Some(tc) = graph.tests.get(tc_id.as_str()) {
            tc_table.push_str(&format!(
                "| {} | {} | {} | {} |\n",
                tc.front.id, tc.front.title, tc.front.test_type, tc.front.status
            ));
        }
    }

    // Per ADR-022, load the base prompt from the per-repo override file
    // when present; fall back to the embedded default otherwise.
    // `prompts::get` honours `[paths].prompts` (FT-057, ADR-048) with a
    // legacy `benchmarks/prompts/` fallback.
    let base_prompt = crate::author::prompts::get(root, config.paths.prompts_resolved(), "implement").unwrap_or_default();

    let dynamic_suffix = format!(
        "# Implementation Task: {} — {}\n\n## Current test status\n{}\n\n## Hard constraints\n- Run the test suite before reporting complete\n- When done: `product verify {}`\n\n## Context Bundle\n{}\n",
        feature.front.id, feature.front.title,
        tc_table,
        feature.front.id,
        bundle,
    );

    let impl_prompt = format!("{}\n\n{}", base_prompt, dynamic_suffix);

    // Write to temp file
    let tmp_dir = std::env::temp_dir();
    let tmp_name = format!("product-impl-{}-{}.md", feature_id, chrono::Utc::now().timestamp());
    let tmp_path = tmp_dir.join(&tmp_name);
    std::fs::write(&tmp_path, &impl_prompt).map_err(|e| {
        ProductError::WriteError {
            path: tmp_path.clone(),
            message: e.to_string(),
        }
    })?;
    println!("OK");
    println!("  Context file: {}", tmp_path.display());

    if dry_run {
        println!();
        println!("  --dry-run: stopping before agent invocation.");
        println!("  Inspect the context file above, then run without --dry-run.");
        return Ok(());
    }

    // Step 4 — Agent invocation
    //
    // The depth-2 context bundle for non-trivial features can exceed Linux's
    // MAX_ARG_STRLEN (128 KB per argv entry), so we can't pass it via
    // `--system-prompt <content>` or as the positional `[prompt]` argument.
    // `--system-prompt-file` only works under `--bare`, which disables
    // OAuth/keychain auth (requires ANTHROPIC_API_KEY) and is not viable.
    //
    // Solution: the bundle is already on disk at `tmp_path`. Spawn claude
    // interactively with a short positional prompt (~200 chars) that tells
    // the agent to Read the bundle file as its first turn. The bundle then
    // arrives as a Read tool-result in the context window, identical effect
    // to delivering it as system prompt, but with no argv pressure.
    //
    // Interactive TUI is preserved end-to-end — drivers (tmux-orchestrator
    // etc.) can attach, capture-pane, send-keys, etc. The agent exits
    // naturally after `product verify` reports passing TCs.
    let kickoff = format!(
        "Read {} — that file is your complete implementation specification \
         (system framing, hard constraints, depth-2 context bundle of the \
         feature plus its linked ADRs and TCs). Follow it to implement {}. \
         When the test suite passes, run `product verify {}`, then exit.",
        tmp_path.display(),
        feature_id,
        feature_id,
    );

    println!(
        "  Step 4: Invoking agent ({})...",
        if headless { "headless" } else { "interactive" }
    );

    let mut args: Vec<&str> = Vec::new();
    if headless {
        args.push("-p");
    }
    args.push("--dangerously-skip-permissions");
    args.push(&kickoff);

    let agent_result = Command::new("claude")
        .args(&args)
        .current_dir(root)
        .status();

    match agent_result {
        Ok(status) => {
            if status.success() {
                println!("  Agent completed successfully.");
            } else {
                println!("  Agent exited with status: {}", status);
            }
        }
        Err(e) => {
            eprintln!("  Warning: could not invoke agent: {}", e);
            eprintln!("  (Is 'claude' in PATH? Or configure a custom agent in product.toml)");
        }
    }
    let _ = impl_prompt; // bundle stays on disk at tmp_path for the agent to Read

    // Step 5 — Auto-verify
    if !no_verify {
        println!("  Step 5: Running verify...");
        run_verify(feature_id, config, root, graph, false)?;
    }

    Ok(())
}
