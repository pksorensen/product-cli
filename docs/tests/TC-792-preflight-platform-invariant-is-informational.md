---
id: TC-792
title: preflight_platform_invariant_is_informational
type: scenario
status: passing
validates:
  features:
  - FT-067
  adrs:
  - ADR-026
phase: 1
runner: cargo-test
runner-args: tc_792_preflight_platform_invariant_is_informational
last-run: 2026-05-26T09:35:27.550025603+00:00
last-run-duration: 0.2s
---

`product preflight FT-X` on a feature that does NOT link a platform-scoped ADR exits 0 (the ADR contributes no gap) and lists the ADR in a *Platform Invariants* informational section.