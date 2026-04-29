---
id: TC-357
title: link_tests_multi_feature
type: scenario
status: passing
validates:
  features: 
  - FT-030
  adrs:
  - ADR-027
phase: 1
runner: cargo-test
runner-args: "tc_357_link_tests_multi_feature"
last-run: 2026-04-28T17:17:27.967937293+00:00
last-run-duration: 0.5s
---

FT-001 and FT-005 both link ADR-002. TC-002 validates ADR-002. Assert TC-002 gains both FT-001 and FT-005.