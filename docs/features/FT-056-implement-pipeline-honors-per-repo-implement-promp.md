---
id: FT-056
title: Implement Pipeline Honors Per-Repo Implement Prompt Override
phase: 5
status: planned
depends-on: []
adrs:
- ADR-022
tests:
- TC-698
- TC-699
domains: []
domains-acknowledged: {}
---

## Description

`benchmarks/prompts/implement-v1.md` is, per ADR-022, a versioned,
repo-owned prompt file that teams may customize. Today the override
flows through two surfaces but not the third:

| Surface | Honors `benchmarks/prompts/implement-v1.md`? |
|---|---|
| `product prompts get implement` (CLI) | ✅ via `author::prompts::get` |
| `mcp__product__product_prompts_get` (MCP) | ✅ same code path |
| `product author feature/adr/review` | ✅ via `author::start_session` |
| **`product implement FT-XXX`** | ❌ inline format string in `pipeline.rs:89-95` |

`src/implement/pipeline.rs::run_implement` builds the agent prompt as
a hard-coded `format!("# Implementation Task: …")` and never consults
`prompts::get(root, "implement")`. Editing the per-repo prompt file
therefore has no effect on what the spawned agent actually sees,
contradicting the design intent of ADR-022 ("System prompts as
versioned files in the repository means they are version-controlled,
reviewable in PRs, and shareable across any agent platform").

This feature wires `pipeline.rs` through `prompts::get` so the
override flows through, mirroring the pattern already used by
`start_session` in `src/author/mod.rs:78-83` (read base prompt from
file or fall back to embedded default, then append dynamic context).

---

## Depends on

None at the artifact level. Implementation builds on the existing
`author::prompts::get` helper.

---

## Scope of this feature

### In

1. **Refactor `pipeline.rs::run_implement`** to call
   `crate::author::prompts::get(root, "implement")` for the base
   prompt, then append the dynamic suffix (feature header, TC status
   table, hard constraints, context bundle). The dynamic suffix
   remains the responsibility of `run_implement` because it depends
   on the live graph.
2. **Preserve fallback**: when `benchmarks/prompts/implement-v1.md`
   is absent, `prompts::get` already falls back to
   `default_content("implement")` (the embedded
   `prompts/implement.txt`). No new fallback logic is needed.
3. **Update embedded default** (`src/author/prompts/implement.txt`)
   to be a useful base prompt the dynamic suffix can sit beneath —
   the current four lines should be expanded modestly to match the
   role description that today lives inline in `pipeline.rs`. The
   dynamic-only content (feature ID, TC table, context bundle) must
   NOT be moved into the embedded default.
4. **Document the composition** in the prompt-template file: a
   short leading section in `implement.txt` makes it explicit that
   `product implement` appends a TC status table, hard constraints,
   and the context bundle after this prompt body, so a user editing
   `implement-v1.md` understands the seam.

### Out

- **Templating syntax** (e.g. `{feature_id}`, `{bundle}` placeholders
  inside the prompt file). A simple base+suffix concatenation is
  sufficient and matches `start_session`. Templated prompts are a
  possible follow-on if real-world use shows the seam needs to move.
- **Reopening the ADR-021 boundary debate.** ADR-021 rejected
  `product implement` as an orchestration command in principle but
  the command exists in the codebase. This feature does not
  re-litigate that decision; it only makes the existing command
  consistent with ADR-022 for as long as `product implement` ships.
- **`author-feature`/`author-adr`/`author-review` paths.** Already
  honor the per-repo override via `start_session`; no change needed.
- **Prompt versioning beyond v1.** The current `implement-v1.md`
  scheme is preserved.

---

## Commands

No new subcommands. Behavior change is internal to
`product implement FT-XXX`.

---

## Implementation notes

- **`src/implement/pipeline.rs`** — replace the inline `format!`
  block (lines 89-95) with:
  1. `let base_prompt = crate::author::prompts::get(root, "implement").unwrap_or_default();`
  2. Build the dynamic suffix (feature header line, TC table, hard
     constraints, context bundle) into a separate `String`.
  3. Concatenate `base_prompt` + `"\n\n"` + `dynamic_suffix` and
     write that to the temp file as today.
- **No new modules.** This is a ~10-line change inside the existing
  `run_implement` function. File-length budget is comfortable.
- **`src/author/prompts/implement.txt`** — modest edit so the
  embedded fallback reads naturally as the prefix of the assembled
  prompt. Keep it under 20 lines.
- **Tests.** A session-style integration test sets up a tempdir
  repo, writes a sentinel string into
  `benchmarks/prompts/implement-v1.md`, runs
  `product implement FT-XXX --dry-run`, parses the
  `Context file: …` line from stdout, reads that file, and asserts
  the sentinel appears at the top. The `--dry-run` path already
  exits before agent invocation so this test does not need
  `claude` in PATH.

---

## Acceptance criteria

A developer can:

1. Run `product prompts init`, edit
   `benchmarks/prompts/implement-v1.md` to add a project-specific
   instruction (e.g. "Always run `cargo fmt` before reporting
   complete"), and observe that `product implement FT-XXX --dry-run`
   writes the customized text into the temp prompt file.
2. Delete `benchmarks/prompts/implement-v1.md` and observe that
   `product implement FT-XXX --dry-run` still produces a prompt
   containing the embedded default, because `prompts::get` falls
   back to `default_content("implement")`.
3. Run `cargo t`, `cargo clippy -- -D warnings -D clippy::unwrap_used`,
   and `cargo build` and observe all pass.

---

## Follow-on work

- **Templated placeholders** — if user feedback shows the
  base+suffix split is too rigid, introduce `{feature_id}`,
  `{tc_table}`, `{bundle}` etc. inside the prompt body. Defer until
  evidence justifies the additional complexity.
- **Audit other inline prompts.** A grep for hard-coded
  `format!("# … Task:` or equivalent should confirm no other
  surface bakes a prompt inline when a versioned file exists for
  it.
