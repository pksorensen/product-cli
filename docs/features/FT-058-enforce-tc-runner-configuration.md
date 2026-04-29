---
id: FT-058
title: Enforce TC Runner Configuration
phase: 5
status: planned
depends-on: []
adrs:
- ADR-013
- ADR-021
- ADR-038
tests:
- TC-705
- TC-706
- TC-707
- TC-708
- TC-709
- TC-710
- TC-711
domains:
- error-handling
- testing
domains-acknowledged: {}
---

## Description

Promote `runner` and `runner-args` from optional TC metadata to a
**required state under active development**, and enforce the invariant
at every gate that can detect it. Today the verify pipeline silently
degrades a no-runner TC to `UNIMPLEMENTED`, which lets a feature stay
perpetually `in-progress` without anyone realising the runner field
was simply forgotten. The system genuinely cannot validate work
without runner configuration; the rules should say so.

The invariant: **every TC linked to a feature whose status is
`in-progress` or `complete` must carry both `runner` and `runner-args`
in its YAML front-matter.** TCs linked only to `planned` or
`abandoned` features are exempt — they are sketches, not executable
specifications.

This feature also addresses the related diagnosis hole: when a runner
*is* configured but the named test function does not exist, cargo
reports "0 tests ran" and the harness misreads it as success. The
verify pipeline already detects the "0 tests ran" pattern; we
extend the diagnostic path so the error names the expected function
and the file it was searched for in.

The single new error class introduced here is **E022 —
TcRunnerMissing**. It carries the list of offending TC IDs so the
user fixes everything in one pass rather than per-TC.

This is a soft amendment of ADR-021's "TCs without a `runner` field
are always `unrunnable`" clause. The clause is preserved for the
`requires`-fails-prerequisite case (genuinely environmental) but
inverted for the no-runner-declared case (genuinely
misconfiguration).

---

## Functional Specification

### Inputs

- The current `KnowledgeGraph` (features, ADRs, TCs) loaded from
  front-matter via `parser::load_all`.
- For verify-time and request-apply checks, the new feature status
  the operation is moving toward (current status for verify; the
  candidate status for a status-transition mutation).
- The TC's front-matter `runner` and `runner-args` fields, parsed via
  the existing `extract_yaml_field` helpers.

### Outputs

- Pass/fail decision per gate, plus on failure a structured
  `ProductError::TcRunnerMissing { tc_ids, feature_id }` value with a
  user-facing rendering matching ADR-013 conventions.
- Exit code 22 (new) when the gate fires from any CLI command.
- Same JSON shape as other E-class errors: `{ "error": "E022", "tc_ids": [...], "feature_id": "..." }`.

### State

- No new persistent state. The check is a pure function over the
  existing graph plus the candidate feature status.
- A small predicate module (`src/tc/runner_required.rs`) exposes:
  - `must_have_runner(tc: &TestCriterion, target_feature_status: FeatureStatus) -> bool`
  - `find_offenders(graph: &KnowledgeGraph, feature_id: &str, target_status: FeatureStatus) -> Vec<String>`

### Behaviour

The invariant fires at four gates:

1. **`product preflight FT-XXX`** — when run against a feature whose
   current status is `in-progress` (or against any feature when
   `--strict` is passed), enumerate every linked TC. If any lacks
   `runner` or `runner-args`, fail with E022 listing all of them.
   This is the gate the harness checks before invoking the agent.

2. **`product feature status FT-XXX in-progress`** and the equivalent
   `product request apply` mutation — refuse the transition if any
   linked TC lacks runner config. The error names the offending TCs
   and gives the canonical fix snippet.

3. **`product graph check`** — emits E022 (fatal, not a W-class
   warning) listing every (feature, tc) pair where the feature is
   `in-progress`/`complete` and the TC lacks runner config. CI
   catches drift introduced by manual edits.

4. **`product verify FT-XXX`** — replaces the current silent
   `UNIMPLEMENTED (no runner configured)` print with a fatal E022
   exit before any TC executes. Defense in depth: even if the other
   three gates were bypassed, verify refuses to claim a feature is
   complete on the basis of "no runner therefore not failing".

In addition, the `cargo test` runner gains a slightly richer
diagnostic: when the runner reports zero tests matched, the existing
`detect_zero_tests` path now emits a message of the form
`"No #[test] fn matching '<runner-args>' found in tests/*.rs — did
you forget to add the integration test?"`. This is independent of
E022; it is a clearer-error follow-on so the *next* class of silent
failure is also surfaced.

### Invariants

- For every `(feature, tc)` edge where `feature.status ∈
  {in-progress, complete}`, `tc.runner` is non-empty AND
  `tc.runner-args` is non-empty.
- The invariant is checked at every gate listed above; no gate is
  optional. (The four-gate redundancy is intentional —
  defense-in-depth against manual YAML edits.)
- A TC linked only to `planned` or `abandoned` features is exempt.
- A TC with `runner` configured but a failing `requires` prerequisite
  remains `unrunnable` (soft) — that case is environmental, not
  configuration.

### Error handling

- `ProductError::TcRunnerMissing { feature_id: String, tc_ids: Vec<String> }`
  → exit code 22.
- The user-facing render follows ADR-013:
  ```
  error[E022]: TC runner configuration missing
    --> docs/tests/TC-XXX-foo.md
    --> docs/tests/TC-YYY-bar.md
     = 2 TC(s) linked to FT-058 lack `runner` and/or `runner-args`
     = hint: add the following to each TC's front-matter:
              runner: cargo-test
              runner-args: "tc_XXX_<snake_case_title>"
     = see ADR-021 §"TC front-matter fields" for the full schema
  ```
- All offenders are reported in one error — never per-TC.
- The error renders identically across all four gates so the user
  sees the same shape regardless of which command tripped it.

### Boundaries

- **In scope:** the four-gate check, the new error class, the richer
  zero-tests-matched diagnostic, the ADR-021 amendment.
- **Out of scope:** auto-deriving runner-args from the TC title (that
  is Design 3 from the proposal — it changes the schema model and is
  a separate decision).
- **Out of scope:** scanning `tests/*.rs` to verify the function
  actually exists at preflight time. The richer cargo diagnostic
  catches this at verify time, which is sufficient given the four
  earlier gates.
- **Out of scope:** changing the behaviour of the `requires` field.
  That branch of "unrunnable" remains exactly as ADR-021 specifies.
- **Out of scope:** retroactive enforcement. Any feature already at
  `complete` with TCs missing runners is grandfathered — `graph
  check` reports them but the migration command for fixing them is a
  follow-on.

---

## Out of scope

- **Convention-derived runner config** (Design 3 from the original
  proposal). Tracked as a possible follow-on; not bundled here
  because it changes the declarative model in ways that require a
  separate ADR amendment and a migration of every existing TC's
  front-matter.
- **Test-function discovery as a preflight check.** The richer
  zero-tests-matched diagnostic at verify time is the agreed scope.
- **Soft-fail flag (`--allow-missing-runner`).** Considered for the
  rollout window and rejected — every TC in the current corpus
  already has runner config (562 TCs, 562 passing per the agent
  context), so there is nothing to grandfather and no need for a
  bypass.
- **Renaming or restructuring `runner` / `runner-args` in the
  schema.** Field names stay; only their required-or-optional
  contract changes (and only conditionally, by feature status).
- **Changing exit codes for unrelated runner-related warnings**
  (W001 "no runnable TCs found", W016 "TCs acknowledged as
  unrunnable"). Those remain warnings.

---

## Implementation notes

- New module `src/tc/runner_required.rs` with the two pure
  predicates listed under State. All four gates call into this
  module; they do not duplicate the predicate logic.
- New error variant `ProductError::TcRunnerMissing { feature_id,
  tc_ids }` in `src/error.rs`, with exit-code mapping `22` and the
  multi-offender rendering shown above.
- Verify-time wiring lives in `src/implement/verify.rs::run_tc_list`:
  the loop's first action becomes "enumerate offenders, return
  early with E022 if any". The current `if tc_runner.is_empty()`
  branch on line 239 is removed.
- Preflight wiring: extend `src/commands/preflight.rs` (or the
  equivalent slice) to load the candidate feature and call
  `find_offenders` with `target_status = feature.status`. If the
  feature is `planned`, the gate is a no-op (matches Boundaries).
- Status-transition wiring: in `src/feature/` the existing
  `plan_status_change` returns a `Plan` describing the transition;
  add a guard that, when the new status is `in-progress`, calls
  `find_offenders` and converts a non-empty result into a refusal
  before any I/O.
- Request-apply wiring: in `src/request/validate.rs`,
  `validate_against_graph` already simulates the post-apply state.
  Reuse that simulated graph: for any feature whose post-apply
  status is `in-progress`/`complete` and whose linked TCs lack
  runners, append an E022 finding.
- Graph-check wiring: in `src/graph/` (or wherever `product graph
  check` walks the graph), add a top-level pass that calls
  `find_offenders` for every feature.
- Richer zero-tests diagnostic in `src/implement/runner.rs`:
  augment `detect_zero_tests` to take `runner_args` so the failure
  message names the expected function. No new error class for this
  branch — it stays under the existing `TcResult::Fail` shape.
- File-length: each touched file stays well under 400 lines after
  the change. The new `runner_required.rs` is ~80 lines including
  unit tests.

---

## Acceptance criteria

A developer can:

1. Author a TC linked to a feature, attempt to transition the
   feature to `in-progress` without configuring `runner`/`runner-args`,
   and observe an E022 error naming every offending TC and the
   exact YAML to paste into each one. Exit code 22.
2. Run `product preflight FT-XXX` against an `in-progress` feature
   with one missing-runner TC and observe the same E022 error
   *before* invoking the agent.
3. Run `product graph check` after manually deleting a `runner` line
   from an existing TC and observe E022 with the offending TC named.
4. Run `product verify FT-XXX` against the same condition and
   observe E022 instead of the previous silent
   `UNIMPLEMENTED (no runner configured)` skip.
5. Configure a TC with `runner: cargo-test` and a `runner-args`
   pointing at a function name that does not exist; run `product
   verify FT-XXX` and observe a clear "No #[test] fn matching
   '...' found" message rather than a green pass.
6. Author a TC linked to a `planned` feature *without* runner
   configuration and observe no error from any gate — this exempt
   case continues to work.
7. Run `cargo t`, `cargo clippy -- -D warnings -D
   clippy::unwrap_used`, and `cargo build` and observe all pass.
