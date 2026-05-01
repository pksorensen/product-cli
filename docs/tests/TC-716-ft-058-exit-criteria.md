---
id: TC-716
title: ft_058_exit_criteria
type: exit-criteria
status: passing
validates:
  features:
  - FT-058
  adrs:
  - ADR-013
  - ADR-021
  - ADR-038
  - ADR-040
phase: 1
runner: cargo-test
runner-args: tc_716_ft_058_exit_criteria
last-run: 2026-04-30T09:25:29.076293260+00:00
last-run-duration: 0.1s
---

## Exit Criteria — FT-058 Enforce TC Runner Configuration

FT-058 is complete when all of the following hold:

1. `product preflight FT-XXX` against an `in-progress` feature with a
   linked TC missing `runner` and/or `runner-args` exits with E022 and
   names every offending TC in one report (TC-709, TC-710).
2. `product feature status FT-XXX in-progress` (and the equivalent
   `product request apply` mutation) refuses the transition with E022
   when any linked TC lacks runner config (TC-708).
3. `product graph check` emits E022 (fatal, not a warning) for every
   `(feature, tc)` pair where the feature is `in-progress`/`complete`
   and the TC lacks runner config (TC-707).
4. `product verify FT-XXX` exits with E022 before any TC executes when
   a linked TC lacks runner config; the prior silent
   `UNIMPLEMENTED (no runner configured)` skip is removed (TC-705).
5. A TC linked only to `planned` (or `abandoned`) features is exempt
   — none of the four gates fires for it (TC-706).
6. The `cargo test` runner emits a clearer "no test function found"
   message naming the expected `runner-args` and the file scanned, so
   the next class of silent failure is also surfaced.
7. The "requires-failed" branch of `unrunnable` continues to behave
   exactly as ADR-021 specifies — environmental prerequisites still
   produce a soft `unrunnable`, not E022 (TC-711).
8. All offenders are reported in a single E022 — never per-TC
   (TC-710).
9. `cargo t`, `cargo clippy -- -D warnings -D clippy::unwrap_used`,
   and `cargo build` all pass.
