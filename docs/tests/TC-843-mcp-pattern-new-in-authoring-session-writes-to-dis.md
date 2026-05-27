---
id: TC-843
title: mcp_pattern_new_in_authoring_session_writes_to_disk
type: scenario
status: passing
validates:
  features:
  - FT-073
  adrs: []
phase: 1
runner: cargo-test
runner-args: tc_843_mcp_pattern_new_in_authoring_session_writes_to_disk
observes:
- file
- mcp-response
last-run: 2026-05-27T14:44:38.372710691+00:00
last-run-duration: 0.3s
---

## Description

Drive an MCP-only `author-pattern` session (no human / CLI
intervention) via a session test. The session invokes
`product_pattern_new` and `product_pattern_link` over MCP.

Assert:

1. After the session, the new PAT file exists on disk with the
   expected front-matter and required body sections.
2. The MCP responses for both calls report success and carry
   the path / writes fields.
3. The TC fails if any MCP write reports success without
   producing a corresponding on-disk artifact — the explicit
   FT-046 anti-pattern guard generalised to patterns.

## Formal specification

⟦Λ:Scenario⟧
Given an MCP-driven author-pattern session,
When the session invokes `product_pattern_new` and
  `product_pattern_link`,
Then both calls produce on-disk writes,
And the TC fails if any call returns success without a disk
  artifact.

⟦Ε⟧⟨δ≜1.0;φ≜1;τ≜◊⁺⟩