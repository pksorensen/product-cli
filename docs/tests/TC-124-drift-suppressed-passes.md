---
id: TC-124
title: drift_suppressed_passes
type: scenario
status: passing
validates:
  features:
  - FT-028
  adrs:
  - ADR-023
phase: 1
runner: cargo-test
runner-args: "tc_124_drift_suppressed_passes"
last-run: 2026-04-28T17:17:23.018590299+00:00
last-run-duration: 0.5s
---

suppress a D002 finding. Run drift check. Assert exit 0.