---
id: TC-798
title: ft_067_exit_criteria
type: exit-criteria
status: passing
validates:
  features:
  - FT-067
  adrs:
  - ADR-025
  - ADR-026
  - ADR-040
phase: 1
runner: cargo-test
runner-args: tc_798_ft_067_exit_criteria
last-run: 2026-05-26T09:35:27.550025603+00:00
last-run-duration: 0.2s
---

## Exit Criteria — FT-067 Platform-scoped ADRs

FT-067 is complete when all of the following hold:

1. `AdrScope::Platform` exists alongside `CrossCutting`, `Domain`, `FeatureSpecific`; YAML serde accepts `"platform"` as a kebab-case value (TC-789).
2. `product adr scope <id> platform` writes `scope: platform` to the ADR front-matter atomically (TC-790).
3. `product adr list --scope platform` returns exactly the platform-scoped ADRs (TC-791).
4. `product preflight FT-X` on a feature that does not link a platform-scoped ADR exits 0 and lists the ADR in a *Platform Invariants* informational section, never as a gap (TC-792).
5. Regression — `product preflight FT-X` on a feature with an unlinked / unacknowledged `cross-cutting` ADR still fails with exit 1 (TC-793).
6. `product gap check` emits G010 (`platform-no-enforcement`, severity low) for any `scope: platform` ADR with no linked TCs (TC-794).
7. Linking any TC to a platform-scoped ADR clears G010 (TC-795).
8. `product adr scope-audit` dry-run lists `cross-cutting → platform` suggestions without modifying files; `--apply` rewrites the `scope:` field atomically per file (TC-796).
9. `product verify --platform` widens its TC selection to include TCs validating any ADR with scope ∈ {cross-cutting, platform} (TC-797).
10. In a fixture repo with 2 cross-cutting + 2 platform + 1 feature-specific ADR, `product preflight FT-X` reports gaps only against the 2 cross-cutting ADRs and lists the 2 platform ADRs under *Platform Invariants*.
11. `cargo build`, `cargo t`, and `cargo clippy -- -D warnings -D clippy::unwrap_used` all pass.