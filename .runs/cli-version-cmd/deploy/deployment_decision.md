```yaml
schema_version: deployment_decision_v1
deployment_verdict: STABLE
gate_verdict: MERGE
default_branch: main

verification:
  ci_workflows: PASS
  branch_protection: PASS
  runtime_verification: PASS
  pre_commit: N/A
  documentation: PASS

failed_checks: []

recommended_actions: []
```

# Deployment Decision

## Evidence

* Gate: `.runs/cli-version-cmd/gate/merge_decision.md` - verdict MERGE, status VERIFIED, recommended_action PROCEED
* CI workflows: `.github/workflows/pack.yml` - 5 jobs defined (lint, pack-check, demoswarm-smoke, runs-tools-tests, doc-drift), triggers on push/PR to main
* Branch protection: GitHub API (`gh api repos/d4nshields/demo-swarm/branches/main/protection`) - required_status_checks enabled with 5 contexts
* Runtime verification: `.runs/cli-version-cmd/deploy/verification_report.md` - smoke_signal: STABLE, 14/14 tests pass

## Rationale

**Governance is now enforced.** Branch protection was enabled on `main` after the initial deploy-decider run. The GitHub API confirms:

1. **Required status checks are enabled** with `strict: true` (require branch to be up to date before merging)
2. **Five required checks configured**: lint, pack-check, demoswarm-smoke, runs-tools-tests, doc-drift
3. **All checks map to CI workflow jobs** defined in `.github/workflows/pack.yml`

This means:
- PRs cannot merge without all five checks passing
- The merge commit (7268525783656c81c236658e6c1aa3a396147b65) for PR #1 was already merged before protection was enabled
- Future merges will enforce the CI gate

**CI workflows are properly configured** with test execution:
- `runs-tools-tests` job runs `cargo test` on the demoswarm-runs-tools package
- `pack-check` job validates pack structure
- `demoswarm-smoke` job runs smoke tests on the CLI
- All jobs trigger on push to main and PRs targeting main

**Runtime verification passed** with smoke_signal: STABLE:
- Merge commit confirmed on origin/main
- Release tag cli-version-cmd-v1 created and published
- 14/14 CLI contract tests pass
- Version subcommand outputs valid JSON

**Documentation exists** and contains dev/CI instructions in README.md and CONTRIBUTING.md.

## Machine Summary

```yaml
status: VERIFIED
recommended_action: PROCEED
route_to_flow: 6
route_to_agent: null
blockers: []
missing_required: []
concerns: []
```
