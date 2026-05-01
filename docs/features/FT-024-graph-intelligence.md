---
id: FT-024
title: Graph Intelligence
phase: 3
status: complete
depends-on:
- FT-016
adrs:
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
  ADR-043: Predates ADR-043; feature command adapters were written before the slice+adapter pattern was formalised and are not retroactively refactored.
  ADR-042: Pre-dates ADR-042; this feature does not define TC types or validate the type vocabulary. FT-048 owns the mechanics.
  ADR-040: predates ADR-040; feature ships independently of the unified verify / LLM-boundary pipeline
  ADR-048: Predates ADR-048; this feature does not author the canonical .product/ folder layout. FT-057 owns the migration command and discovery fallback.
  ADR-018: Predates the 2026-04-22 scope promotion of ADR-018 to cross-cutting. Test coverage reflects the property/session/benchmark strategy as it existed when this feature shipped; not retroactively reclassified.
  ADR-041: Pre-dates ADR-041; this feature does not author absence TCs or set removes/deprecates on ADRs. FT-047 owns the mechanics.
  ADR-047: Predates ADR-047; this feature does not author the functional-spec body convention. FT-055 owns the structural validator and W030 mechanics.
---

Structural graph analysis that goes beyond navigation — centrality ranking, SPARQL queries, and graph statistics.

### Betweenness Centrality

```
product graph central              # top-10 ADRs by betweenness centrality
product graph central --top N      # configurable N
product graph central --all        # full ranked list
```

Uses Brandes' algorithm for betweenness centrality. ADRs within context bundles are ordered by centrality descending by default — the most structurally important decisions appear first. Pass `--order id` to override.

Centrality scores are included in the TTL export on `product graph rebuild`.

### SPARQL Queries

```
product graph query "SELECT ..."   # SPARQL 1.1 over the generated graph
```

Uses embedded Oxigraph (ADR-008) for SPARQL query execution against the TTL-exported graph.

### Graph Statistics

```
product graph stats                # artifact counts, link density, centrality summary,
                                   # phi (formal block coverage) across test criteria
```

### Exit Criteria

`product graph central` returns ADR-001 as rank 1 on the PiCloud graph. Centrality computation completes in < 100ms on 200 nodes. Impact analysis completes in < 50ms.

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
