---
id: TC-789
title: scope_platform_parses_and_round_trips
type: scenario
status: passing
validates:
  features:
  - FT-067
  adrs:
  - ADR-025
phase: 1
runner: cargo-test
runner-args: tc_789_scope_platform_parses_and_round_trips
last-run: 2026-05-26T09:35:27.550025603+00:00
last-run-duration: 0.2s
---

`scope: platform` parses, round-trips through the front-matter, and does not cause `product graph check` to fail. Regression guard on the new `AdrScope::Platform` variant.