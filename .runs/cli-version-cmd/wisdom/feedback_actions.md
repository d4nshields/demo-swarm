# Feedback Actions (Run cli-version-cmd)

## Outcome Snapshot
- issue_drafts: 3
- suggestions: 7
- inputs_present:
  - learnings: yes
  - regressions: yes
  - artifact_audit: yes

## Flow 1 -- Signal (Proposed edits)

- [ ] SUG-001: Add MUST/SHOULD/MAY markers to requirements template
  - evidence: .runs/cli-version-cmd/wisdom/learnings.md#Learning: Requirements
  - proposed_change: `.claude/agent/signal/requirements-author.md` -> Requirements Template section -> Add instruction: "Each requirement MUST include a priority marker: MUST (critical), SHOULD (important), or MAY (optional). Example: 'REQ-001 (MUST): Version Subcommand Existence'"

- [ ] SUG-002: Add testability field to BDD scenario template
  - evidence: .runs/cli-version-cmd/wisdom/learnings.md#Learning: Build
  - proposed_change: `.claude/agent/signal/bdd-author.md` -> Scenario Template section -> Add field: "testability: direct | implicit | not-testable" with instruction: "Annotate each scenario with testability classification. If not 'direct', include explanation."

- [ ] SUG-003: Add Operational Risks category to early_risks.md template
  - evidence: .runs/cli-version-cmd/wisdom/learnings.md#Learning: Design
  - proposed_change: `.claude/agent/signal/risk-assessor.md` -> early_risks.md template -> Add new section: "## Operational Risks" with categories: credentials/authentication, permissions/access control, branch protection/CI requirements, external service dependencies

## Flow 2 -- Plan (Proposed edits)

- [ ] SUG-004: Add Environment Prerequisites section to ADR template
  - evidence: .runs/cli-version-cmd/wisdom/learnings.md#Learning: Design
  - proposed_change: `.claude/agent/plan/design-architect.md` -> ADR Template section -> Add section after "Non-Goals": "## Environment Prerequisites" with subsections: Required tooling (coverage tools, linters, security scanners), CI/CD requirements, External dependencies

- [ ] SUG-005: Add open question resolution checkpoint to Plan cleanup
  - evidence: .runs/cli-version-cmd/wisdom/learnings.md#Learning: Requirements
  - proposed_change: `.claude/agent/plan/plan-cleanup.md` -> Pre-receipt checklist -> Add step: "Review all open questions from Signal and Plan. For each OQ, ensure one of: (a) explicit answer recorded, (b) 'A: accepted suggested default' noted, or (c) escalation reason documented."

## Flow 3 -- Build (Issue drafts + suggestions)

- ISSUE: ISSUE-DRAFT-001: Add TDD checkpoint to Build flow expecting initial test failures
  - target_flow: 3
  - labels: enhancement, dx, flow-3-build
  - summary: Build flow should include an explicit TDD checkpoint that expects tests to fail before implementation, validating the red-green-refactor cycle. This run demonstrated clean TDD execution (8 passed, 6 failed pre-implementation) but there was no formal checkpoint to verify this pattern.
  - acceptance_criteria:
    - [ ] Build flow includes a "TDD validation" step after test authoring
    - [ ] Step expects and documents initial test failures (non-zero exit acceptable at this stage)
    - [ ] Step records failure count for comparison after implementation
    - [ ] test_changes_summary.md template includes "Pre-implementation failures: N" field
  - evidence:
    - .runs/cli-version-cmd/wisdom/learnings.md#Learning: Build
    - .runs/cli-version-cmd/build/test_changes_summary.md

## Flow 5 -- Deploy (Issue drafts)

- ISSUE: ISSUE-DRAFT-002: Add credential verification to Flow 5 preflight checks
  - target_flow: 5
  - labels: enhancement, security, flow-5-deploy
  - summary: Flow 5 should verify GitHub credentials match the expected account before attempting push/merge operations. This run experienced a credential mismatch ("danshieldspala" vs "d4nshields") that caused push failure during Gate checkpoint. Early detection would fail fast with clear remediation guidance.
  - acceptance_criteria:
    - [ ] Flow 5 preflight includes `gh auth status` verification
    - [ ] Preflight includes `git remote -v` check against expected repository
    - [ ] Mismatch produces clear error message with remediation steps
    - [ ] Credential verification result recorded in deployment_log.md
  - evidence:
    - .runs/cli-version-cmd/wisdom/learnings.md#Surprises
    - .runs/cli-version-cmd/gate/git_status.md

- ISSUE: ISSUE-DRAFT-003: Add branch protection check to Flow 5 preflight
  - target_flow: 5
  - labels: enhancement, governance, flow-5-deploy
  - summary: Flow 5 should check branch protection status before attempting merge, not after. This run succeeded functionally (PR merged, release published) but returned NOT_DEPLOYED verdict due to branch protection not being enabled. Preflight check would surface this earlier and allow remediation before merge attempt.
  - acceptance_criteria:
    - [ ] Flow 5 preflight includes GitHub API check for branch protection on target branch
    - [ ] Missing branch protection produces warning with severity level (not hard block)
    - [ ] Preflight result recorded in deployment_decision.md under "Governance Preflight" section
    - [ ] Warning does not block merge if explicit override provided
  - evidence:
    - .runs/cli-version-cmd/wisdom/learnings.md#Surprises
    - .runs/cli-version-cmd/deploy/deployment_decision.md

## Cross-cutting (Optional)

- [ ] SUG-006: Add coverage tooling to environment prerequisites documentation
  - evidence: .runs/cli-version-cmd/gate/coverage_audit.md#Machine Summary
  - proposed_change: `docs/how-to/customize-pack.md` -> Prerequisites section -> Add: "Coverage tooling: cargo-tarpaulin or cargo-llvm-cov required for Rust projects. Install via: `cargo install cargo-tarpaulin`"

- [ ] SUG-007: Document credential verification as standard preflight check
  - evidence: .runs/cli-version-cmd/wisdom/learnings.md#Actions
  - proposed_change: `CLAUDE.md` -> Two-Gate Prerequisites section -> Add note: "Before running flows that involve GitHub operations, verify credentials: `gh auth status` should show expected account; `git remote -v` should show expected repository."

## Issues Created
None. (Drafts only; no GitHub side effects.)

## Actions Deferred

- RSK-003 (pack-check version inconsistency): Follow-up run needed
  - reason: Out of scope for this run; requires separate feature implementation to add version subcommand to pack-check tool

- Coverage tooling installation: Environment setup task
  - reason: Requires environment changes outside of pack/flow modifications; marked as RSK-006 ACCEPTED for this run

- Branch protection enablement: Repository configuration task
  - reason: Requires repository admin access; governance decision outside pack scope

## Inventory (machine countable)
(Only these prefixed lines; do not rename prefixes)

- ISSUE_DRAFT: ISSUE-DRAFT-001 target_flow=3 labels="enhancement, dx, flow-3-build"
- ISSUE_DRAFT: ISSUE-DRAFT-002 target_flow=5 labels="enhancement, security, flow-5-deploy"
- ISSUE_DRAFT: ISSUE-DRAFT-003 target_flow=5 labels="enhancement, governance, flow-5-deploy"
- SUGGESTION: SUG-001 target_flow=1
- SUGGESTION: SUG-002 target_flow=1
- SUGGESTION: SUG-003 target_flow=1
- SUGGESTION: SUG-004 target_flow=2
- SUGGESTION: SUG-005 target_flow=2
- SUGGESTION: SUG-006 target_flow=cross-cutting
- SUGGESTION: SUG-007 target_flow=cross-cutting

## Machine Summary
```yaml
status: VERIFIED
recommended_action: PROCEED
route_to_flow: null
route_to_agent: null
blockers: []
missing_required: []
concerns:
  - Issue drafts require human review before GitHub creation
  - Template changes (SUG-001 through SUG-005) should be validated against existing agent prompts
  - Environment prerequisites (SUG-006, SUG-007) are documentation-only and do not enforce tooling presence
```
