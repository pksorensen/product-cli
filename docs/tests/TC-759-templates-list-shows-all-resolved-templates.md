---
id: TC-759
title: templates-list-shows-all-resolved-templates
type: scenario
status: unimplemented
validates:
  features: []
  adrs:
  - ADR-049
phase: 1
runner: cargo-test
runner-args: tc_759_templates_list_shows_all_resolved_templates
---

## Scenario — `templates-list-shows-all-resolved-templates`

**Given** the six built-in templates plus one user template at `~/.product/templates/team-bundle.toml` and one repo template at `.product/templates/pr-review.toml`,
**When** the user runs `product context templates`,
**Then** stdout lists all eight names with their descriptions and source markers (`(built-in)`, `(user)`, `(repo)`); a `Default target:` footer reports the currently configured default and where it came from (`from product.toml` or `fallback`).

## Validates

- FT-063 — Per-Model Context Bundle Templates (`templates` list command)
- ADR-049 — Per-Model Context Bundle Templates as Data Files
