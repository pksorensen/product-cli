---
id: FT-016
title: Graph Model
phase: 1
status: complete
depends-on: []
adrs:
- ADR-003
- ADR-008
- ADR-012
tests:
- TC-009
- TC-010
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
- TC-157
- TC-232
- TC-233
- TC-234
- TC-235
- TC-236
- TC-237
- TC-238
- TC-249
domains:
- data-model
domains-acknowledged:
  ADR-042: Pre-dates ADR-042; this feature does not define TC types or validate the type vocabulary. FT-048 owns the mechanics.
  ADR-043: Predates ADR-043; feature command adapters were written before the slice+adapter pattern was formalised and are not retroactively refactored.
  ADR-040: predates ADR-040; feature ships independently of the unified verify / LLM-boundary pipeline
  ADR-048: Predates ADR-048; this feature does not author the canonical .product/ folder layout. FT-057 owns the migration command and discovery fallback.
  ADR-047: Predates ADR-047; this feature does not author the functional-spec body convention. FT-055 owns the structural validator and W030 mechanics.
  ADR-018: Predates the 2026-04-22 scope promotion of ADR-018 to cross-cutting. Test coverage reflects the property/session/benchmark strategy as it existed when this feature shipped; not retroactively reclassified.
  ADR-041: Pre-dates ADR-041; this feature does not author absence TCs or set removes/deprecates on ADRs. FT-047 owns the mechanics.
---

Product builds an in-memory directed graph from front-matter on every invocation. The graph is also exportable as RDF Turtle via `product graph rebuild`.

### Edge Types

| Edge | From | To | Description |
|---|---|---|---|
| `implementedBy` | Feature | ADR | Feature is governed by this decision |
| `validatedBy` | Feature | TestCriterion | Feature is verified by this test |
| `testedBy` | ADR | TestCriterion | Decision is verified by this test |
| `supersedes` | ADR | ADR | This decision replaces another |
| `depends-on` | Feature | Feature | Implementation dependency — must complete before |

The reverse of every edge is implicit. Impact analysis (`product impact`) traverses the reverse graph to compute reachability.

### Graph Algorithms

| Algorithm | Applied to | Command | Purpose |
|---|---|---|---|
| Topological sort (Kahn's) | Feature `depends-on` DAG | `product feature next` | Correct implementation ordering |
| BFS to depth N | All edges | `product context --depth N` | Transitive context assembly |
| Betweenness centrality (Brandes') | ADR nodes | `product graph central` | Structural importance ranking |
| Reverse-graph BFS | All edges reversed | `product impact` | Change impact analysis |

### RDF Export

Product exports the knowledge graph as RDF Turtle. The ontology prefix is `pm:` (product-meta).

```turtle
@prefix pm: <https://product-meta/ontology#> .
@prefix ft: <https://product-meta/feature/> .
@prefix adr: <https://product-meta/adr/> .
@prefix tc: <https://product-meta/test/> .

ft:FT-001 a pm:Feature ;
    pm:title "Cluster Foundation" ;
    pm:phase 1 ;
    pm:status pm:InProgress ;
    pm:implementedBy adr:ADR-001 ;
    pm:implementedBy adr:ADR-002 ;
    pm:validatedBy tc:TC-001 ;
    pm:validatedBy tc:TC-002 .

ft:FT-003 a pm:Feature ;
    pm:dependsOn ft:FT-001 ;
    pm:dependsOn ft:FT-002 .

adr:ADR-002 a pm:ArchitecturalDecision ;
    pm:title "openraft for Cluster Consensus" ;
    pm:status pm:Accepted ;
    pm:betweennessCentrality 0.731 ;
    pm:appliesTo ft:FT-001 ;
    pm:testedBy tc:TC-002 .

tc:TC-002 a pm:TestCriterion ;
    pm:title "Raft Leader Election" ;
    pm:type pm:Scenario ;
    pm:status pm:Unimplemented ;
    pm:validates ft:FT-001 ;
    pm:validates adr:ADR-002 .
```

Betweenness centrality scores are written into the TTL export on `graph rebuild` so external SPARQL tools can query on them.

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
