---
id: TC-790
title: adr_scope_platform_writes_field
type: scenario
status: passing
validates:
  features:
  - FT-067
  adrs:
  - ADR-025
phase: 1
runner: cargo-test
runner-args: tc_790_adr_scope_platform_writes_field
last-run: 2026-05-26T09:35:27.550025603+00:00
last-run-duration: 0.1s
---

`product adr scope <id> platform` writes `scope: platform` into the ADR's front-matter atomically.