# Deploy Cleanup Report

## Run: cli-version-cmd
## Completed: 2025-12-17T20:08:58Z

## Machine Summary
status: UNVERIFIED
recommended_action: ESCALATE
route_to_flow: null
route_to_agent: null
deployment_verdict: NOT_DEPLOYED
gate_verdict: MERGE
missing_required: []
missing_optional: [flow_plan.md]
blockers:
  - deployment_verdict is NOT_DEPLOYED (push permission denied prevents remote operations)
  - branch_protection verification is UNKNOWN (critical check)
  - push permission denied prevents remote operations
concerns:
  - 6 local commits on run/cli-version-cmd remain unpushed
  - No PR was created; no merge to mainline occurred
  - Gate verdict MERGE could not be executed due to auth barrier

## Artifact Verification
| Artifact | Status |
|----------|--------|
| deployment_decision.md | Found |
| deployment_log.md | Found |
| verification_report.md | Found |
| flow_plan.md | Missing (optional) |

## Extracted (anchored)
- deployment_verdict: NOT_DEPLOYED (from deployment_decision.md YAML block)
- gate_verdict: MERGE (from deployment_decision.md YAML block)
- deploy_decider status: UNVERIFIED (from deployment_decision.md Machine Summary)
- deploy_decider recommended_action: ESCALATE (from deployment_decision.md Machine Summary)

## Counts Derived (stable markers)
| Metric | Value | Source |
|--------|-------|--------|
| failed_checks | 1 | deployment_decision.md YAML (`- check:` items) |
| ci_checks_total | 0 | verification_report.md (DEP_CI_RUN markers) |
| deploy_events_total | 0 | verification_report.md (DEP_DEPLOY_EVENT markers) |
| verification_checks_total | 0 | ci_checks_total + deploy_events_total |

## Signals Extracted (from verification_report.md)
| Signal | Value | Source |
|--------|-------|--------|
| ci_signal | N/A | DEP_CI_SIGNAL marker |
| deploy_signal | FAIL | DEP_DEPLOY_SIGNAL marker |
| not_deployed | yes | DEP_NOT_DEPLOYED marker |

## Quality Gates (from Machine Summary status fields)
| Gate | Status | Source |
|------|--------|--------|
| deploy_decider | UNVERIFIED | deployment_decision.md Machine Summary |
| verification_report | UNVERIFIED | verification_report.md Machine Summary |

## Index Updated
- Fields changed: status, last_flow, updated_at
- status: UNVERIFIED
- last_flow: deploy
- updated_at: 2025-12-17T20:08:58Z

## Notes

This deployment did not complete due to authentication failure. The gate verdict was MERGE (approved), but push to remote was denied (403 permission denied). Local smoke verification confirmed STABLE signal with 14/14 tests passing.

The implementation is complete and functionally correct. Authentication resolution is required before re-running Flow 5 to complete the deployment.
