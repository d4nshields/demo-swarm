# Gate Cleanup Report

## Run: cli-version-cmd
## Completed: 2025-12-17T19:46:44Z

## Machine Summary
status: UNVERIFIED
recommended_action: ESCALATE
route_to_flow: null
route_to_agent: null
merge_verdict: MERGE
missing_required: []
missing_optional: [flow_plan.md]
blockers:
  - merge_verdict is MERGE but coverage_audit status is UNVERIFIED (coverage tooling not available in environment; RSK-006 ACCEPTED by merge-decider)
  - coverage_line_percent is null (no stable markers; coverage instrumentation not run)
  - coverage_branch_percent is null (no stable markers; coverage instrumentation not run)
concerns:
  - receipt_checks_total/checks_passed not present in receipt_audit.md Machine Summary (severity_summary available instead)
  - findings_total not present in security_scan.md Machine Summary (severity_summary shows 0/0/0)
  - policy violations_total not present in policy_analysis.md Machine Summary (compliance_summary shows non_compliant=0)

## Artifact Verification
| Artifact | Status |
|----------|--------|
| merge_decision.md | Found |
| receipt_audit.md | Found |
| contract_compliance.md | Found |
| security_scan.md | Found |
| coverage_audit.md | Found |
| policy_analysis.md | Found |
| risk_assessment.md | Found |
| gate_fix_summary.md | Found |
| flow_plan.md | Missing (optional) |

## Extracted Gate Statuses (Machine Summary)
| Check | Status | Source |
|-------|--------|--------|
| merge_decider | VERIFIED | merge_decision.md |
| receipt_audit | VERIFIED | receipt_audit.md |
| contract_compliance | VERIFIED | contract_compliance.md |
| security_scan | VERIFIED | security_scan.md |
| coverage_audit | UNVERIFIED | coverage_audit.md |

## Counts Derived (Stable Markers)
| Metric | Value | Source |
|--------|-------|--------|
| receipt_checks_total | null | receipt_audit.md (field not present; severity_summary shows 0/0/0) |
| receipt_checks_passed | null | receipt_audit.md (field not present) |
| contract_violations | 0 | contract_compliance.md (violations_total) |
| security_findings | 0 | security_scan.md (severity_summary critical=0 major=0 minor=0) |
| policy_violations | 0 | policy_analysis.md (compliance_summary non_compliant=0) |
| coverage_line_percent | null | coverage_audit.md (coverage_line_percent: null - no instrumentation) |
| coverage_branch_percent | null | coverage_audit.md (coverage_branch_percent: null - no instrumentation) |

## Index Updated
- Fields changed: status, last_flow, updated_at
- status: UNVERIFIED
- last_flow: gate
- updated_at: 2025-12-17T19:46:44Z

## Notes

The gate cleanup determined status=UNVERIFIED because:
1. coverage_audit has status=UNVERIFIED (coverage tooling not available in environment)
2. coverage_line_percent and coverage_branch_percent are null (no coverage instrumentation output)

Per the pack's status model, VERIFIED requires all required gate statuses to be VERIFIED and required counts to be non-null. Since coverage_audit is UNVERIFIED and coverage metrics are null, the overall gate status is UNVERIFIED.

The merge-decider has issued verdict=MERGE with status=VERIFIED, having evaluated the coverage gap and accepted RSK-006. The recommended_action is ESCALATE because there is a nominal inconsistency (MERGE verdict with UNVERIFIED sub-gate), which requires human judgment to proceed.

The merge-decider's analysis in merge_decision.md explicitly addresses this:
- RSK-006 (coverage tooling limitation) is ACCEPTED
- The implementation is trivially safe (~30 lines, no user input, no I/O)
- All 14 tests pass, including 11 version-specific integration tests
- All other gate checks are VERIFIED with PROCEED recommendation

Human escalation can evaluate whether to proceed with Flow 5 (Deploy) given the risk acceptance documented in the gate artifacts.
