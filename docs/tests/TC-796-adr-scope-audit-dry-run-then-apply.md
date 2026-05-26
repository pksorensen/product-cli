---
id: TC-796
title: adr_scope_audit_dry_run_then_apply
type: scenario
status: passing
validates:
  features:
  - FT-067
  adrs:
  - ADR-025
phase: 1
runner: cargo-test
runner-args: tc_796_adr_scope_audit_dry_run_then_apply
last-run: 2026-05-26T09:35:27.550025603+00:00
last-run-duration: 0.2s
---

`product adr scope-audit` dry-run prints suggestions for cross-cutting → platform re-classification and leaves files unchanged. `--apply` rewrites the `scope:` field atomically per file.