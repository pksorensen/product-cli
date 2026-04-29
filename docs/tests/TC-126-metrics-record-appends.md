---
id: TC-126
title: metrics_record_appends
type: scenario
status: passing
validates:
  features:
  - FT-028
  adrs:
  - ADR-024
phase: 1
runner: cargo-test
runner-args: "tc_126_metrics_record_appends"
last-run: 2026-04-28T17:17:23.018590299+00:00
last-run-duration: 0.4s
---

run `product metrics record` twice. Assert `metrics.jsonl` has two lines and both are valid JSON with all required fields.