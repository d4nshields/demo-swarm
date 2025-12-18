# Regression Report

## Machine Summary
status: VERIFIED

recommended_action: PROCEED
route_to_flow: null
route_to_agent: null

blockers: []

missing_required: []

concerns:
  - Coverage thresholds (80% line, 70% branch) cannot be numerically verified due to missing tooling (RSK-006 ACCEPTED)
  - Branch protection not enforced on main (deployment concern, not regression)

severity_summary:
  critical: 0
  major: 0
  minor: 0

regressions_found: 0
baseline_available: no

## Context
- flow: wisdom
- run_id: cli-version-cmd
- issue_number: 1
- inputs_used:
  - .runs/cli-version-cmd/run_meta.json
  - .runs/cli-version-cmd/build/test_critique.md
  - .runs/cli-version-cmd/build/build_receipt.json
  - .runs/cli-version-cmd/build/test_execution.md
  - .runs/cli-version-cmd/build/test_output.log
  - .runs/cli-version-cmd/build/test_changes_summary.md
  - .runs/cli-version-cmd/build/code_critique.md
  - .runs/cli-version-cmd/build/self_review.md
  - .runs/cli-version-cmd/gate/coverage_audit.md
  - .runs/cli-version-cmd/gate/merge_decision.md
  - .runs/cli-version-cmd/gate/risk_assessment.md
  - .runs/cli-version-cmd/deploy/deploy_receipt.json

## Canonical Test Summary
- pytest_summary: "test result: ok. 14 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out"
- source: .runs/cli-version-cmd/build/test_critique.md

## Test Analysis

| Metric | Value | Source |
|--------|-------|--------|
| Total Tests | 14 | build/test_critique.md |
| Passed | 14 | build/test_critique.md |
| Failed | 0 | build/test_critique.md |
| XFailed | 0 | build/test_critique.md |
| Skipped | 0 | build/test_critique.md |
| Flaky | 0 | unknown (no flakiness evidence) |

### Pre-existing Tests (Baseline)
3 pre-existing tests in `cli_contract.rs` continued to pass:
- `ms_get_missing_file_returns_null_and_zero_exit`
- `invalid_regex_does_not_break_contract`
- `count_pattern_honors_null_if_zero_flag`

### New Tests Added
11 version-specific tests added by this run, all passing.

### Intermediate TDD Failures (Expected, Resolved)
`test_output.log` captured an intermediate state showing 6 failures:
- `version_json_contains_version_field`
- `version_json_contains_name_field`
- `version_json_is_pretty_printed`
- `version_success_writes_stdout_only`
- `version_matches_cargo_toml`
- `version_flag_and_subcommand_coexist`

These were **expected TDD failures** (tests written before implementation). The `test_changes_summary.md` explicitly documents: "Test Run Results: 8 passed; 6 failed (expected pre-implementation)". All failures were resolved once the `version.rs` implementation was completed.

## Regression Register

| ID | Severity | Test/Area | Summary | Blamed Commit | Related Issue |
|----|----------|-----------|---------|---------------|---------------|
| (none) | - | - | No regressions detected | - | - |

## Regression Details

No regressions found. This was a new feature implementation with TDD approach. All pre-existing tests continue to pass, and all new tests pass after implementation.

## Coverage Signals

| Source | Finding | Notes |
|--------|---------|------|
| gate/coverage_audit.md | UNKNOWN | Thresholds: 80% line, 70% branch; not measurable due to missing cargo-tarpaulin |
| gate/risk_assessment.md | ACCEPTED | RSK-006 - coverage tooling limitation accepted by platform team |

### Coverage Assessment
- **Numeric coverage unavailable**: cargo-tarpaulin and llvm-cov not installed in environment
- **Functional coverage comprehensive**: 14/14 tests pass, 11 version-specific scenarios covered
- **Risk status**: RSK-006 ACCEPTED - environment limitation, not code quality issue
- **Changed surface**: version.rs (~26 lines), mod.rs (2 lines), main.rs (1 line dispatch)

## Issue Correlation

| Issue | Related Regression | Confidence | Notes |
|-------|-------------------|------------|-------|
| #1 | none | N/A | Feature request issue; no regressions to correlate |

GitHub issue #1 is the feature request for this implementation. Repository has issues disabled on d4nshields/demo-swarm fork; EffortlessMetrics/demo-swarm issue could not be fetched (GraphQL error).

## Blame Summary

| Commit | Author | Date | Files | Related Regressions |
|--------|--------|------|-------|---------------------|
| 091fa24 | Dan Shields | 2025-12-17 | 3 (version.rs, mod.rs, main.rs) | none |

All implementation files were introduced in a single commit by Dan Shields. No prior code was modified in a way that could introduce regressions.

## Stability Assessment

### Test Stability: STABLE
- No flakiness detected
- No test rerun evidence in artifacts
- All tests are deterministic (version output determinism explicitly tested)
- TDD cycle completed cleanly (failures to passes documented)

### Implementation Stability: STABLE
- Compile-time constants only (no runtime variability)
- No I/O, no network, no external dependencies
- Security scan: trivially safe attack surface

### Quality Gate Status
- test_critic: VERIFIED
- code_critic: VERIFIED (minor: NFR-COMP-002 doc gap resolved)
- self_reviewer: VERIFIED
- merge_decision: MERGE
- deploy: PR #1 merged, release `cli-version-cmd-v1` published

### Deployment Outcome
- smoke_signal: STABLE
- merge_sha: 7268525783656c81c236658e6c1aa3a396147b65
- release_url: https://github.com/d4nshields/demo-swarm/releases/tag/cli-version-cmd-v1
- Branch protection concern: no required status checks on main (operational improvement, not regression)

## Recommended Next
- PROCEED - No regressions found, all quality gates passed
- Consider installing cargo-tarpaulin for future runs to enable numeric coverage metrics
- Consider enabling branch protection with required status checks on main branch
- RSK-003 (pack-check version inconsistency) can be addressed in follow-up run

---

## Regression Analyst Result
status: VERIFIED
recommended_action: PROCEED
route_to_flow: null
route_to_agent: null
severity_summary:
  critical: 0
  major: 0
  minor: 0
regressions_found: 0
blockers: []
missing_required: []
