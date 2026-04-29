---
id: TC-362
title: graph_infer_general
type: scenario
status: passing
validates:
  features: 
  - FT-030
  adrs:
  - ADR-027
phase: 1
runner: cargo-test
runner-args: "tc_362_graph_infer_general"
last-run: 2026-04-28T17:17:27.967937293+00:00
last-run-duration: 0.3s
---

add FT-009 → ADR-021 link. Run `product graph infer --feature FT-009`. Assert TC-041 and TC-042 (which validate ADR-021) gain FT-009 in their features list.