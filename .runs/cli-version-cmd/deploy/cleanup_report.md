# Deploy Cleanup Report

## Run: cli-version-cmd
## Completed: 2025-12-18T02:15:20Z

## Machine Summary
status: VERIFIED
recommended_action: PROCEED
route_to_flow: null
route_to_agent: null
deployment_verdict: STABLE
gate_verdict: MERGE
missing_required: []
missing_optional: []
blockers: []
concerns: []

## Artifact Verification
| Artifact | Status |
|----------|--------|
| deployment_decision.md | Found |
| deployment_log.md | Found |
| verification_report.md | Found |
| flow_plan.md | Found |

## Extracted (anchored)
- deployment_verdict: STABLE (from deployment_decision.md YAML block)
- gate_verdict: MERGE (from deployment_decision.md YAML block)
- deploy_decider status: VERIFIED (from deployment_decision.md Machine Summary)
- deploy_decider recommended_action: PROCEED (from deployment_decision.md Machine Summary)

## Counts Derived (stable markers)
| Metric | Value | Source |
|--------|-------|--------|
| failed_checks | 0 | deployment_decision.md YAML (`- check:` items) |
| ci_checks_total | 1 | verification_report.md (DEP_CI_RUN markers) |
| deploy_events_total | 1 | verification_report.md (DEP_DEPLOY_EVENT markers) |
| verification_checks_total | 2 | ci_checks_total + deploy_events_total |

## Signals Extracted (from verification_report.md)
| Signal | Value | Source |
|--------|-------|--------|
| ci_signal | N/A | DEP_CI_SIGNAL marker |
| deploy_signal | N/A | DEP_DEPLOY_SIGNAL marker |
| not_deployed | no | DEP_NOT_DEPLOYED marker |

## Merge and Release Details
| Field | Value |
|-------|-------|
| merge_completed | true |
| pr_number | 1 |
| merge_sha | 7268525783656c81c236658e6c1aa3a396147b65 |
| release_created | true |
| tag | cli-version-cmd-v1 |
| release_url | https://github.com/d4nshields/demo-swarm/releases/tag/cli-version-cmd-v1 |

## Quality Gates
| Gate | Status |
|------|--------|
| deploy_decider | VERIFIED |
| verification_report | VERIFIED |

## Smoke Signal
smoke_signal: STABLE

Basis:
- 14/14 CLI contract tests pass
- Version subcommand executes correctly with valid JSON output
- Branch protection now enabled on main with 5 required status checks
- Release published and accessible

## Index Updated
- Fields changed: status, last_flow, updated_at
- status: VERIFIED
- last_flow: deploy
- updated_at: 2025-12-18T02:15:20Z

## Summary

Flow 5 (Deploy) completed successfully with STABLE verdict. The run PR #1 was merged to main, tag `cli-version-cmd-v1` was created, and GitHub release published. Branch protection has been enabled on main with required status checks, addressing the prior governance concern. All verification checks passed with smoke_signal: STABLE.

Next recommended: Flow 6 (Wisdom) to extract learnings and close the feedback loop.
