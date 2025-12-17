# Coverage Audit for cli-version-cmd

## Machine Summary
```yaml
status: UNVERIFIED
recommended_action: BOUNCE
route_to_flow: 3
route_to_agent: test-author
blockers:
  - Line coverage threshold (80%) cannot be verified - no coverage report produced
  - Branch coverage threshold (70%) cannot be verified - no coverage report produced
missing_required:
  - Coverage instrumentation output (e.g., cargo-tarpaulin, llvm-cov report)
concerns:
  - Critical path coverage expectation (version.rs) cannot be verified without per-file coverage data
# Numeric fields for gate-cleanup
coverage_line_percent: null
coverage_branch_percent: null
thresholds_defined: yes
```

## Sources Consulted

* `.runs/cli-version-cmd/plan/test_plan.md` - coverage thresholds and critical path definition
* `.runs/cli-version-cmd/build/test_execution.md` - test run results (no coverage percentages)
* `.runs/cli-version-cmd/build/build_receipt.json` - build status and test summary
* `.runs/cli-version-cmd/build/impl_changes_summary.md` - changed files inventory
* `.runs/cli-version-cmd/build/test_changes_summary.md` - scenario coverage (qualitative)

## Thresholds (from Plan)

```yaml
thresholds_status: PRESENT
line_required: 80
branch_required: 70
critical_path_defined: yes
critical_path_pointer: "test_plan.md line 44: tools/demoswarm-runs-tools/src/commands/version.rs"
```

Plan also notes (line 47-49):
- measurement_notes: Use cargo test --workspace with coverage tooling (e.g., cargo-tarpaulin or llvm-cov)
- The version.rs module is small (~30 lines) and should achieve near-100% coverage
- Critical path coverage (90%) applies to version.rs

## Coverage Evidence Found

* `.runs/cli-version-cmd/build/test_execution.md` - reports test pass/fail counts only (14 passed, 0 failed); no line/branch percentage metrics
* `.runs/cli-version-cmd/build/build_receipt.json` - confirms test results; no coverage percentage fields
* `.runs/cli-version-cmd/build/test_changes_summary.md` - documents scenario coverage qualitatively (11 scenarios covered); no numeric coverage

**Searched and NOT found:**
* `coverage.xml`, `cobertura.xml`, `jacoco.xml` - not present
* `lcov.info` - not present
* `coverage.json`, `coverage-summary.json`, `coverage-final.json` - not present
* `*coverage*.html` - not present (only agent prompt file found)

## Results (mechanical)

```yaml
line_actual: null
branch_actual: null
evidence_consistency: unknown
```

| Metric | Required | Actual | Status  | Evidence                                            |
| ------ | -------: | -----: | ------- | --------------------------------------------------- |
| Line   |       80 |   null | UNKNOWN | no coverage instrumentation output found            |
| Branch |       70 |   null | UNKNOWN | no coverage instrumentation output found            |

## Changed-Surface Focus

From `impl_changes_summary.md`, the following files were changed:

| File | Change Type |
|------|-------------|
| `tools/demoswarm-runs-tools/src/commands/version.rs` | ADDED |
| `tools/demoswarm-runs-tools/src/commands/mod.rs` | CHANGED |
| `tools/demoswarm-runs-tools/src/main.rs` | CHANGED |

Per-file coverage data is not available from any evidence source.

## Critical Path Coverage

Plan declares critical path coverage expectation:
- `COVERAGE_CRITICAL_PATH: tools/demoswarm-runs-tools/src/commands/version.rs`
- Expectation: 90% coverage for version.rs (per test_plan.md line 49)

**Status: UNVERIFIABLE**

No per-file or per-module coverage report exists. The test execution report confirms tests exercise the version subcommand functionally (11 version-specific tests pass), but this does not provide the numeric line/branch coverage required by Plan.

**What would make this verifiable:**
- Run tests with coverage instrumentation: `cargo tarpaulin --out Xml` or `cargo llvm-cov --output-path coverage.json`
- Parse coverage report for version.rs specifically
- Report line and branch percentages for the critical path file

## Findings

### CRITICAL

(none)

### MAJOR

* [MAJOR] COV-MAJ-001: Plan specifies line coverage threshold (80%) but Build did not produce coverage instrumentation output
  * Evidence: test_plan.md line 42 "COVERAGE_LINE_REQUIRED: 80"; test_execution.md contains no coverage percentage

* [MAJOR] COV-MAJ-002: Plan specifies branch coverage threshold (70%) but Build did not produce coverage instrumentation output
  * Evidence: test_plan.md line 43 "COVERAGE_BRANCH_REQUIRED: 70"; test_execution.md contains no coverage percentage

* [MAJOR] COV-MAJ-003: Plan declares critical path coverage expectation for version.rs but no per-file coverage data available
  * Evidence: test_plan.md line 44 "COVERAGE_CRITICAL_PATH: tools/demoswarm-runs-tools/src/commands/version.rs"; no coverage report found in `.runs/cli-version-cmd/build/`

### MINOR

* [MINOR] COV-MIN-001: test_plan.md specifies coverage measurement approach (cargo-tarpaulin or llvm-cov) but test-runner did not invoke with coverage flags
  * Evidence: test_plan.md line 47 "measurement_notes: Use cargo test --workspace with coverage tooling"; test_execution.md shows plain "cargo test" invocation

## Notes for Merge-Decider

Coverage thresholds are clearly defined in Plan (line 80%, branch 70%, critical path 90% for version.rs), but Build did not produce coverage instrumentation output. All 14 tests pass, and scenario coverage appears comprehensive (11 of 16 planned scenarios implemented, with remaining 5 documented as edge cases or implicitly covered). However, the numeric coverage requirements cannot be verified without running tests with coverage tooling.

**Recommendation:** BOUNCE to Flow 3 (test-author or test-runner) to:
1. Re-run tests with coverage instrumentation enabled (e.g., `cargo tarpaulin` or `cargo llvm-cov`)
2. Produce a coverage report artifact
3. Include coverage percentages in test_execution.md or a separate coverage_report.md

Alternatively, if coverage instrumentation is not available in this environment, the merge-decider may ESCALATE for human judgment on whether functional test coverage (14 passing tests, 11 scenarios) is sufficient despite missing numeric metrics.

## Inventory (machine countable)

- COV_MAJOR: COV-MAJ-001
- COV_MAJOR: COV-MAJ-002
- COV_MAJOR: COV-MAJ-003
- COV_MINOR: COV-MIN-001
- COV_METRIC: line required=80 actual=null status=UNKNOWN
- COV_METRIC: branch required=70 actual=null status=UNKNOWN
- COV_THRESHOLD_STATUS: PRESENT
