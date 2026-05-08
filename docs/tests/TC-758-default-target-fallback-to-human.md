---
id: TC-758
title: default-target-fallback-to-human
type: scenario
status: unimplemented
validates:
  features: []
  adrs:
  - ADR-049
phase: 1
runner: cargo-test
runner-args: tc_758_default_target_fallback_to_human
---

## Scenario — `default-target-fallback-to-human`

**Given** `product.toml` without a `[context]` section (no `default-target` set),
**When** the user runs `product context FT-XXX`,
**Then** the bundle is rendered using the `human` template (Markdown, no framing).

Confirms the backward-compat invariant: `product context FT-XXX` without flags on a fresh repo produces terminal-readable Markdown.

## Validates

- FT-063 — Per-Model Context Bundle Templates (fallback default)
- ADR-049 — Per-Model Context Bundle Templates as Data Files
