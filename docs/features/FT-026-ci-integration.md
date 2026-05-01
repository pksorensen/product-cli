---
id: FT-026
title: CI Integration
phase: 3
status: complete
depends-on:
- FT-018
- FT-024
adrs:
- ADR-009
- ADR-013
tests:
- TC-181
domains:
- api
- error-handling
domains-acknowledged:
  ADR-042: Pre-dates ADR-042; this feature does not define TC types or validate the type vocabulary. FT-048 owns the mechanics.
  ADR-043: Predates ADR-043; feature command adapters were written before the slice+adapter pattern was formalised and are not retroactively refactored.
  ADR-047: Predates ADR-047; this feature does not author the functional-spec body convention. FT-055 owns the structural validator and W030 mechanics.
  ADR-018: Predates the 2026-04-22 scope promotion of ADR-018 to cross-cutting. Test coverage reflects the property/session/benchmark strategy as it existed when this feature shipped; not retroactively reclassified.
  ADR-040: predates ADR-040; feature ships independently of the unified verify / LLM-boundary pipeline
  ADR-041: Pre-dates ADR-041; this feature does not author absence TCs or set removes/deprecates on ADRs. FT-047 owns the mechanics.
  ADR-048: Predates ADR-048; this feature does not author the canonical .product/ folder layout. FT-057 owns the migration command and discovery fallback.
---

Machine-readable output formats and CI/CD integration points that make Product a first-class CI gate.

### JSON Output

`--format json` output on all list and navigation commands. Structured JSON to stdout for CI annotation and tooling integration.

```
product feature list --format json
product graph check --format json     # structured stderr for CI
product gap check --format json       # structured JSON to stdout for CI annotation
```

### Shell Completions

```
product completions bash > /etc/bash_completion.d/product
product completions zsh > ~/.zfunc/_product
product completions fish > ~/.config/fish/completions/product.fish
```

### GitHub Actions

Example GitHub Actions workflow that gates PRs on:
- `product graph check --format json` — zero errors
- `product metrics threshold` — fitness functions within bounds
- `product gap check --changed --format json` — no new gaps

### Exit Criteria

`product graph check` CI gate fails on a PR with a broken link. All list commands produce valid JSON with `--format json`.

---

## Description

See existing prose above. This heading is a backfilled stub for ADR-047 structural compliance; the substantive description for this legacy feature lives in the prose preceding this section.

## Functional Specification

This feature predates ADR-047. Subsections below are backfilled stubs to satisfy structural completeness; substantive behaviour is documented in the prose above and in the linked ADRs.

### Inputs

Not separately enumerated — this feature predates ADR-047. See the prose above and linked ADRs for substantive content.

### Outputs

Not separately enumerated — this feature predates ADR-047. See the prose above and linked ADRs for substantive content.

### State

Not separately enumerated — this feature predates ADR-047. See the prose above and linked ADRs for substantive content.

### Behaviour

Not separately enumerated — this feature predates ADR-047. See the prose above and linked ADRs for substantive content.

### Invariants

Not separately enumerated — this feature predates ADR-047. See the prose above and linked ADRs for substantive content.

### Error handling

Not separately enumerated — this feature predates ADR-047. See the prose above and linked ADRs for substantive content.

### Boundaries

Not separately enumerated — this feature predates ADR-047. See the prose above and linked ADRs for substantive content.

## Out of scope

Not separately enumerated for this legacy feature; scope boundaries are implicit in the prose above and in the linked ADRs.
