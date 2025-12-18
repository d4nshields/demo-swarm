# Deploy Cleanup Report

## Run: cli-version-cmd
## Completed: 2025-12-18T00:58:15Z

## Machine Summary
status: UNVERIFIED
recommended_action: BOUNCE
route_to_flow: 3
route_to_agent: null
deployment_verdict: NOT_DEPLOYED
gate_verdict: MERGE
missing_required: []
missing_optional: [flow_plan.md]
blockers:
  - branch_protection: FAIL - no required status checks configured on main
concerns:
  - Merge succeeded without governance enforcement - protection should be enabled before future merges
  - CI workflows exist but are not enforced via required status checks

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
- deploy_decider status: VERIFIED (from deployment_decision.md Machine Summary)
- deploy_decider recommended_action: BOUNCE (from deployment_decision.md Machine Summary)

## Counts Derived (stable markers)
| Metric | Value | Source |
|--------|-------|--------|
| failed_checks | 1 | deployment_decision.md YAML (`- check:` items) |
| ci_checks_total | 1 | verification_report.md (DEP_CI_RUN markers) |
| deploy_events_total | 1 | verification_report.md (DEP_DEPLOY_EVENT markers) |
| verification_checks_total | 2 | ci_checks_total + deploy_events_total |

## Signals Extracted (from verification_report.md)
| Signal | Value | Source |
|--------|-------|--------|
| ci_signal | N/A | DEP_CI_SIGNAL marker |
| deploy_signal | N/A | DEP_DEPLOY_SIGNAL marker |
| not_deployed | no | DEP_NOT_DEPLOYED marker |

## Quality Gates (from Machine Summary status fields)
| Gate | Status | Source |
|------|--------|--------|
| deploy_decider | VERIFIED | deployment_decision.md Machine Summary |
| verification_report | VERIFIED | verification_report.md Machine Summary |

## Merge + Release Status
| Item | Status | Details |
|------|--------|---------|
| merge_completed | true | PR #1 merged to main |
| merge_sha | 7268525783656c81c236658e6c1aa3a396147b65 | merge commit |
| release_created | true | GitHub Release published |
| tag | cli-version-cmd-v1 | release tag |
| smoke_signal | STABLE | 14/14 tests pass |

## Index Updated
- Fields changed: status, last_flow, updated_at
- status: UNVERIFIED
- last_flow: deploy
- updated_at: 2025-12-18T00:58:15Z

## Notes

This deployment completed successfully from a release/merge perspective:
- PR #1 was created and merged to main
- Tag cli-version-cmd-v1 was created
- GitHub Release was published
- Smoke signal is STABLE (14/14 tests pass)

However, the deployment_verdict is NOT_DEPLOYED because branch protection is not enabled on the main branch. The GitHub API reports "Branch not protected" (HTTP 404). This is a governance concern, not a code quality issue.

The code itself is production-ready and has been successfully released. The UNVERIFIED status reflects that governance enforcement (branch protection with required status checks) is not configured, meaning future merges could bypass CI checks.

Recommended action: Enable branch protection on main via GitHub Settings to achieve STABLE verdict on subsequent runs.
