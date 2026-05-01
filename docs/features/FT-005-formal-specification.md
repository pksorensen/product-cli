---
id: FT-005
title: Formal Specification
phase: 1
status: complete
depends-on: []
adrs:
- ADR-011
- ADR-015
tests:
- TC-066
- TC-067
- TC-068
- TC-069
- TC-070
- TC-161
domains:
- data-model
- storage
domains-acknowledged:
  ADR-043: Predates ADR-043; feature command adapters were written before the slice+adapter pattern was formalised and are not retroactively refactored.
  ADR-040: predates ADR-040; feature ships independently of the unified verify / LLM-boundary pipeline
  ADR-048: Predates ADR-048; this feature does not author the canonical .product/ folder layout. FT-057 owns the migration command and discovery fallback.
  ADR-018: Predates the 2026-04-22 scope promotion of ADR-018 to cross-cutting. Test coverage reflects the property/session/benchmark strategy as it existed when this feature shipped; not retroactively reclassified.
  data-model: Formal types and invariants describe data constraints but do not define persistent storage schemas. ADR-015 (file write safety) governs the write path; formal blocks are parsed in-memory per ADR-011/ADR-016.
---

⟦Σ:Types⟧{
  Node≜IRI
  Role≜Leader|Follower|Learner
  ClusterState≜⟨nodes:Node+, roles:Node→Role⟩
}

⟦Λ:Scenario⟧{
  given≜cluster_init(nodes:2)
  when≜elapsed(10s)
  then≜∃n∈nodes: roles(n)=Leader
       ∧ graph_contains(n, picloud:hasRole, picloud:Leader)
}

⟦Ε⟧⟨δ≜0.95;φ≜100;τ≜◊⁺⟩
```

**Invariant example:**
```markdown
---
id: TC-020
title: Betweenness Centrality Always In Range
type: invariant
status: unimplemented
validates:
  features: [FT-001]
  adrs: [ADR-012]
phase: 3
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
