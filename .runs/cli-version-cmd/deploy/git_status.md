# Git Status

## Status: COMPLETED_WITH_ANOMALY
## Operation: checkpoint

## Before
- Branch: run/cli-version-cmd
- Head: 506f49dc5236210202f0af23310ba029c2922b88
- Porcelain: 9 modified, 2 untracked

## Allowlist
- .runs/cli-version-cmd/deploy/
- .runs/cli-version-cmd/run_meta.json
- .runs/index.json

## Staged Files
- .runs/cli-version-cmd/deploy/cleanup_report.md
- .runs/cli-version-cmd/deploy/deploy_receipt.json
- .runs/cli-version-cmd/deploy/deployment_decision.md
- .runs/cli-version-cmd/deploy/deployment_log.md
- .runs/cli-version-cmd/deploy/git_status.md
- .runs/cli-version-cmd/deploy/secrets_scan.md
- .runs/cli-version-cmd/deploy/secrets_status.json
- .runs/cli-version-cmd/deploy/verification_report.md
- .runs/cli-version-cmd/run_meta.json
- .runs/index.json

## Unexpected Paths (anomaly)
- .runs/cli-version-cmd/gate/git_status.md (untracked)

## Actions Taken
- Reset staging area
- Staged only allowlist files
- Committed allowlist files (audit trail preserved)
- Skipped push due to anomaly (tighten-only safety)

## After
- Branch: run/cli-version-cmd
- Head: 773bc8044ae8a4e65405d0d50c0913c8b5b67364
- Porcelain: 1 untracked (anomaly file)

## Notes
- Anomaly path `.runs/cli-version-cmd/gate/git_status.md` is from a previous flow (gate)
- This is outside the deploy flow allowlist
- Push skipped per tighten-only safety policy
- proceed_to_github_ops: false due to anomaly
