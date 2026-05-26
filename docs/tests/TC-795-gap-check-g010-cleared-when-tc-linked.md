---
id: TC-795
title: gap_check_g010_cleared_when_tc_linked
type: scenario
status: passing
validates:
  features:
  - FT-067
  adrs:
  - ADR-025
phase: 1
runner: cargo-test
runner-args: tc_795_gap_check_g010_cleared_when_tc_linked
last-run: 2026-05-26T09:35:27.550025603+00:00
last-run-duration: 0.1s
---

Linking any TC (invariant, scenario, absence, or other) to a platform-scoped ADR clears G010 — Product never re-emits the gap once an enforcement TC exists.