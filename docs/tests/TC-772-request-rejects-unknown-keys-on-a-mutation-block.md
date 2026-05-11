---
id: TC-772
title: request rejects unknown keys on a mutation block
type: scenario
status: passing
validates:
  features:
  - FT-064
  adrs: []
phase: 5
runner: cargo-test
runner-args: "tc_772_request_rejects_unknown_keys_on_a_mutation_block"
last-run: 2026-05-11T09:30:05.870828163+00:00
last-run-duration: 0.2s
---

A mutation block carrying a key outside the closed set
`{op, field, value}` (for example `path:`, `to:`, `from:`) is
rejected with an E-class finding pointing at the offending key.
Today the unknown key is silently dropped at parse time and the
mutation either applies a malformed action or applies nothing.