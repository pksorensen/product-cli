---
id: TC-791
title: adr_list_scope_platform_filter
type: scenario
status: passing
validates:
  features:
  - FT-067
  adrs:
  - ADR-025
phase: 1
runner: cargo-test
runner-args: tc_791_adr_list_scope_platform_filter
last-run: 2026-05-26T09:35:27.550025603+00:00
last-run-duration: 0.2s
---

`product adr list --scope platform` returns exactly the platform-scoped ADRs — cross-cutting, domain, and feature-specific ADRs are excluded.