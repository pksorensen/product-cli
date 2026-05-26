---
id: TC-793
title: preflight_cross_cutting_still_gates
type: scenario
status: passing
validates:
  features:
  - FT-067
  adrs:
  - ADR-026
phase: 1
runner: cargo-test
runner-args: tc_793_preflight_cross_cutting_still_gates
last-run: 2026-05-26T09:35:27.550025603+00:00
last-run-duration: 0.1s
---

Regression — `product preflight FT-X` on a feature linking a `cross-cutting` ADR still treats unlinked-and-unacknowledged cross-cutting ADRs as gaps and fails with exit 1.