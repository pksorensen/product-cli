---
id: FT-011
title: Context Bundle Format
phase: 1
status: complete
depends-on: []
adrs:
- ADR-006
- ADR-008
- ADR-012
tests:
- TC-016
- TC-017
- TC-018
- TC-019
- TC-020
- TC-024
- TC-025
- TC-026
- TC-041
- TC-042
- TC-043
- TC-044
- TC-045
- TC-046
- TC-047
- TC-048
- TC-049
- TC-050
- TC-051
- TC-052
- TC-053
- TC-054
- TC-158
- TC-201
- TC-202
- TC-203
- TC-205
- TC-232
- TC-233
- TC-234
- TC-235
- TC-236
- TC-237
- TC-238
- TC-249
domains:
- api
- data-model
domains-acknowledged:
  ADR-048: Predates ADR-048; this feature does not author the canonical .product/ folder layout. FT-057 owns the migration command and discovery fallback.
  ADR-042: Pre-dates ADR-042; this feature does not define TC types or validate the type vocabulary. FT-048 owns the mechanics.
  ADR-043: Predates ADR-043; feature command adapters were written before the slice+adapter pattern was formalised and are not retroactively refactored.
  ADR-041: Pre-dates ADR-041; this feature does not author absence TCs or set removes/deprecates on ADRs. FT-047 owns the mechanics.
  ADR-047: Predates ADR-047; this feature does not author the functional-spec body convention. FT-055 owns the structural validator and W030 mechanics.
  ADR-040: predates ADR-040; feature ships independently of the unified verify / LLM-boundary pipeline
  ADR-018: Predates the 2026-04-22 scope promotion of ADR-018 to cross-cutting. Test coverage reflects the property/session/benchmark strategy as it existed when this feature shipped; not retroactively reclassified.
---

The context command assembles a deterministic markdown bundle. Order is always: feature → ADRs (by ID ascending) → test criteria (by phase, then type: exit-criteria, scenario, invariant, chaos).

The bundle opens with an AISP-influenced formal header block (see ADR-011) that an agent can parse without reading the full document. It declares the bundle's identity, all linked artifact IDs, and aggregate evidence metrics derived from the test criteria evidence blocks.

```markdown
# Context Bundle: FT-001 — Cluster Foundation

⟦Ω:Bundle⟧{
  feature≜FT-001:Feature
  phase≜1:Phase
  status≜InProgress:FeatureStatus
  generated≜2026-04-11T09:00:00Z
  implementedBy≜⟨ADR-001,ADR-002,ADR-003,ADR-006⟩:Decision+
  validatedBy≜⟨TC-001,TC-002,TC-003,TC-004⟩:TestCriterion+
}
⟦Ε⟧⟨δ≜0.92;φ≜75;τ≜◊⁺⟩

---

```

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
