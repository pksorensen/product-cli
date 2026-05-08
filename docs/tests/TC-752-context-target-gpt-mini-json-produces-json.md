---
id: TC-752
title: context-target-gpt-mini-json-produces-json
type: scenario
status: unimplemented
validates:
  features: []
  adrs:
  - ADR-049
phase: 1
runner: cargo-test
runner-args: tc_752_context_target_gpt_mini_json_produces_json
---

## Scenario — `context-target-gpt-mini-json-produces-json`

**Given** the built-in `gpt-mini-json` template (`[format].structure = "json"`),
**When** the user runs `product context FT-XXX --target gpt-mini-json`,
**Then** stdout is a single valid JSON object. The keys are exactly the entries of `[ordering].sections`. ADR / TC bodies are encoded as Markdown strings inside the JSON (`content_format = "markdown"`).

`serde_json::from_str` parses the output without error.

## Validates

- FT-063 — Per-Model Context Bundle Templates (JSON rendering)
- ADR-049 — Per-Model Context Bundle Templates as Data Files
