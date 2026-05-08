---
id: TC-766
title: for-llm-flag-is-deprecated-alias-for-target
type: scenario
status: unimplemented
validates:
  features: []
  adrs:
  - ADR-049
phase: 1
runner: cargo-test
runner-args: tc_766_for_llm_flag_is_deprecated_alias_for_target
---

## Scenario — `for-llm-flag-is-deprecated-alias-for-target`

**Given** a CLI invocation `product context FT-XXX --for-llm`,
**When** the command runs,
**Then** stdout contains the same bundle that `product context FT-XXX --target claude-opus` produces (byte-identical), and stderr contains a deprecation note pointing to `--target`.

Passing both `--for-llm` and `--target` simultaneously emits **E028 conflicting-target-flags** without rendering anything.

## Validates

- FT-063 — Per-Model Context Bundle Templates (deprecation alias)
- ADR-049 — Per-Model Context Bundle Templates as Data Files
