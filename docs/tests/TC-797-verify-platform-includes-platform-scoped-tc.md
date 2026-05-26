---
id: TC-797
title: verify_platform_includes_platform_scoped_tc
type: scenario
status: passing
validates:
  features:
  - FT-067
  adrs:
  - ADR-040
phase: 1
runner: cargo-test
runner-args: tc_797_verify_platform_includes_platform_scoped_tc
last-run: 2026-05-26T09:35:27.550025603+00:00
last-run-duration: 0.2s
---

`product verify --platform` widens its TC selection to include TCs validating any ADR with scope ∈ {cross-cutting, platform}. A TC linked only to a platform-scoped ADR runs through the platform verify stage just like a cross-cutting one.