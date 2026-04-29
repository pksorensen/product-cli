---
id: TC-359
title: link_tests_idempotent
type: scenario
status: passing
validates:
  features: 
  - FT-030
  adrs:
  - ADR-027
phase: 1
runner: cargo-test
runner-args: "tc_359_link_tests_idempotent"
last-run: 2026-04-28T17:17:27.967937293+00:00
last-run-duration: 0.4s
---

run `product migrate link-tests` twice. Assert file content identical after both runs. Assert second run reports "0 new links."