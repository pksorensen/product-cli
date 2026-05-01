---
id: FT-008
title: Schema Migration
phase: 2
status: complete
depends-on:
- FT-003
adrs:
- ADR-002
- ADR-014
- ADR-016
tests:
- TC-060
- TC-061
- TC-062
- TC-063
- TC-064
- TC-065
- TC-179
domains:
- data-model
domains-acknowledged:
  ADR-043: Predates ADR-043; feature command adapters were written before the slice+adapter pattern was formalised and are not retroactively refactored.
  ADR-041: Pre-dates ADR-041; this feature does not author absence TCs or set removes/deprecates on ADRs. FT-047 owns the mechanics.
  ADR-040: predates ADR-040; feature ships independently of the unified verify / LLM-boundary pipeline
  ADR-018: Predates the 2026-04-22 scope promotion of ADR-018 to cross-cutting. Test coverage reflects the property/session/benchmark strategy as it existed when this feature shipped; not retroactively reclassified.
  ADR-042: Pre-dates ADR-042; this feature does not define TC types or validate the type vocabulary. FT-048 owns the mechanics.
  ADR-048: Predates ADR-048; this feature does not author the canonical .product/ folder layout. FT-057 owns the migration command and discovery fallback.
  ADR-047: Predates ADR-047; this feature does not author the functional-spec body convention. FT-055 owns the structural validator and W030 mechanics.
---

In-place schema upgrades for front-matter when the schema version changes.

```
product migrate schema --dry-run    # report what would change without writing
product migrate schema --execute    # update all files in place
```

The `schema-version` field in `product.toml` declares the current schema version. On startup, Product validates:
- E008 — forward incompatibility (file schema version > binary schema version)
- W007 — upgrade available (file schema version < binary schema version)

Migration functions are registered per version transition (e.g., v0→v1). Each migration function transforms front-matter in place while preserving unknown fields. Concurrent `product migrate schema` commands are prevented by advisory locking (E010).

### Exit Criteria

Run `product migrate schema` on a v0 repository — all files updated, `schema-version` bumped. Run two concurrent commands — one succeeds, one exits E010. No data corruption.

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
