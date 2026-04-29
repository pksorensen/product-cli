---
id: TC-131
title: metrics_jsonl_merge_conflict_safe
type: scenario
status: passing
validates:
  features:
  - FT-028
  adrs:
  - ADR-024
phase: 1
runner: cargo-test
runner-args: "tc_131_metrics_jsonl_merge_conflict_safe"
last-run: 2026-04-28T17:17:23.018590299+00:00
last-run-duration: 0.5s
---

create `metrics.jsonl` with two records on the same line (simulating a bad merge). Assert `product metrics trend` handles it gracefully with a W-class warning.