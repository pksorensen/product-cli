---
id: TC-794
title: gap_check_platform_no_enforcement
type: scenario
status: passing
validates:
  features:
  - FT-067
  adrs:
  - ADR-025
phase: 1
runner: cargo-test
runner-args: tc_794_gap_check_platform_no_enforcement
last-run: 2026-05-26T09:35:27.550025603+00:00
last-run-duration: 0.2s
---

`product gap check` emits the new G010 gap when an ADR carries `scope: platform` and has no linked TC. The rationale: a platform-scoped ADR is "enforced by the platform itself," which is meaningful only if an enforcement TC exists.