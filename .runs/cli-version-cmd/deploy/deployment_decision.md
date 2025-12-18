```yaml
schema_version: deployment_decision_v1
deployment_verdict: NOT_DEPLOYED
gate_verdict: MERGE
default_branch: main

verification:
  ci_workflows: PASS
  branch_protection: FAIL
  runtime_verification: PASS
  pre_commit: N/A
  documentation: PASS

failed_checks:
  - check: branch_protection
    status: FAIL
    reason: "GitHub API reports 'Branch not protected' (HTTP 404); no required status checks configured on main"

recommended_actions:
  - "Enable branch protection on main via GitHub Settings -> Branches -> Add rule"
  - "Configure required status checks to include: lint, pack-check, demoswarm-smoke, runs-tools-tests, doc-drift"
  - "Re-run deploy-decider after protection is enabled to achieve STABLE verdict"
```

# Deployment Decision

## Evidence

* Gate: `.runs/cli-version-cmd/gate/merge_decision.md` (verdict: MERGE, status: VERIFIED, recommended_action: PROCEED)
* CI workflows: `.github/workflows/pack.yml` (jobs: lint, pack-check, demoswarm-smoke, runs-tools-tests, doc-drift)
* Branch protection: `gh api repos/d4nshields/demo-swarm/branches/main/protection` returned HTTP 404 "Branch not protected"
* Runtime verification: `.runs/cli-version-cmd/deploy/verification_report.md` (status: VERIFIED, smoke_signal: STABLE)
* Default branch: main (from `git symbolic-ref refs/remotes/origin/HEAD`)

## Rationale

Gate verdict is MERGE with VERIFIED status and PROCEED recommendation. All gate checks passed except coverage tooling (risk-accepted per RSK-006).

CI workflows are properly configured. The repository has `.github/workflows/pack.yml` with five jobs:
- `lint`: runs Python scripts for portable claude checks and frontmatter linting
- `pack-check`: runs `bash .claude/scripts/pack-check.sh`
- `demoswarm-smoke`: installs CLI and runs smoke tests
- `runs-tools-tests`: runs `cargo test` on the runs-tools crate
- `doc-drift`: checks for documentation drift

Runtime verification passed: merge completed successfully (SHA 7268525783656c81c236658e6c1aa3a396147b65), release published (tag cli-version-cmd-v1), smoke signal is STABLE, 14/14 tests pass.

However, branch protection is not enabled on main. The GitHub API explicitly returns "Branch not protected" (HTTP 404). This means:
- PRs can be merged without passing CI checks
- Force pushes to main are not blocked
- Required reviewers are not enforced

Without branch protection, governance is not enforced at the GitHub level. The merge that occurred (PR #1) succeeded because no protection rules blocked it, not because protection rules were satisfied. This is a distinction that matters for governance verification.

The verdict is NOT_DEPLOYED because branch protection (a critical check) is FAIL. The recommended action is BOUNCE to enable branch protection, which is a repo-owned configuration change that does not require code changes.

## Machine Summary

```yaml
status: VERIFIED
recommended_action: BOUNCE
route_to_flow: 3
route_to_agent: null
blockers:
  - "branch_protection: FAIL - no required status checks configured on main"
missing_required: []
concerns:
  - "Merge succeeded without governance enforcement - protection should be enabled before future merges"
  - "CI workflows exist but are not enforced via required status checks"
```
