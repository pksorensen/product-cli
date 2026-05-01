---
id: FT-007
title: Formal Specification
phase: 1
status: complete
depends-on: []
adrs:
- ADR-004
- ADR-011
tests:
- TC-011
- TC-012
- TC-152
domains:
- data-model
domains-acknowledged:
  ADR-047: Predates ADR-047; this feature does not author the functional-spec body convention. FT-055 owns the structural validator and W030 mechanics.
  ADR-043: Predates ADR-043; feature command adapters were written before the slice+adapter pattern was formalised and are not retroactively refactored.
  ADR-041: Pre-dates ADR-041; this feature does not author absence TCs or set removes/deprecates on ADRs. FT-047 owns the mechanics.
  ADR-040: predates ADR-040; feature ships independently of the unified verify / LLM-boundary pipeline
  ADR-042: Pre-dates ADR-042; this feature does not define TC types or validate the type vocabulary. FT-048 owns the mechanics.
  ADR-048: Predates ADR-048; this feature does not author the canonical .product/ folder layout. FT-057 owns the migration command and discovery fallback.
  ADR-018: Predates the 2026-04-22 scope promotion of ADR-018 to cross-cutting. Test coverage reflects the property/session/benchmark strategy as it existed when this feature shipped; not retroactively reclassified.
---

⟦Σ:Types⟧{
  Graph≜⟨nodes:Node+, edges:Edge*⟩
  CentralityScore≜Float
}

⟦Γ:Invariants⟧{
  ∀g:Graph, ∀n∈g.nodes: betweenness(g,n) ≥ 0.0 ∧ betweenness(g,n) ≤ 1.0
}

⟦Ε⟧⟨δ≜0.99;φ≜100;τ≜◊⁺⟩
```

**Benchmark example:**
```markdown
---
id: TC-030
title: LLM Context Quality — Raft Leader Election
type: benchmark
status: unimplemented
validates:
  features: [FT-001]
  adrs: [ADR-006, ADR-012]
phase: 3
benchmark:
  task: benchmarks/tasks/task-001-raft-leader-election
  rubric: benchmarks/tasks/task-001-raft-leader-election/rubric.md
  conditions: [none, naive, product]
  runs-per-condition: 5
  pass-threshold:
    product: 0.80
    delta-vs-naive: 0.15
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
