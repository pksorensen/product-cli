---
id: FT-020
title: Migration Path
phase: 1
status: complete
depends-on: []
adrs:
- ADR-014
- ADR-017
tests:
- TC-060
- TC-061
- TC-062
- TC-063
- TC-064
- TC-065
- TC-080
- TC-081
- TC-082
- TC-083
- TC-084
- TC-085
- TC-162
- TC-275
domains:
- api
- data-model
domains-acknowledged:
  ADR-048: Predates ADR-048; this feature does not author the canonical .product/ folder layout. FT-057 owns the migration command and discovery fallback.
  ADR-047: Predates ADR-047; this feature does not author the functional-spec body convention. FT-055 owns the structural validator and W030 mechanics.
  ADR-041: Pre-dates ADR-041; this feature does not author absence TCs or set removes/deprecates on ADRs. FT-047 owns the mechanics.
  ADR-043: Predates ADR-043; feature command adapters were written before the slice+adapter pattern was formalised and are not retroactively refactored.
  ADR-042: Pre-dates ADR-042; this feature does not define TC types or validate the type vocabulary. FT-048 owns the mechanics.
  ADR-040: predates ADR-040; feature ships independently of the unified verify / LLM-boundary pipeline
  ADR-018: Predates the 2026-04-22 scope promotion of ADR-018 to cross-cutting. Test coverage reflects the property/session/benchmark strategy as it existed when this feature shipped; not retroactively reclassified.
---

Migration is a two-phase extract-then-confirm process. See ADR-017 for full heuristic specification.

```bash
# Dry run — see what would be created
product migrate from-adrs picloud-adrs.md --validate
product migrate from-prd picloud-prd.md --validate

# Execute — write files, skip existing
product migrate from-adrs picloud-adrs.md --execute
product migrate from-prd picloud-prd.md --execute

# Interactive — review each artifact before writing
product migrate from-prd picloud-prd.md --interactive

# Post-migration: fill in link gaps and generate checklist
product graph check
product checklist generate
```

The migration parser uses heading structure to detect artifact boundaries and extracts phase references, status markers, and test criteria from subsections. It does not infer `depends-on` edges or feature→ADR links — those require human review and are filled in via `product feature link` commands after migration.

The source document is never modified. Migration can be re-run safely.

---

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
