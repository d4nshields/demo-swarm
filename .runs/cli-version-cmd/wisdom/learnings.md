# Learnings from Run: cli-version-cmd

## Outcome Snapshot
- Gate verdict: MERGE (all checks pass except coverage tooling; RSK-006 ACCEPTED)
- Deploy outcome: NOT_DEPLOYED (branch protection not configured; merge and release succeeded functionally)
- Regression count: 0 (no regressions detected; 14/14 tests passing)

---

## Learning: Requirements

### What Worked
- **Explicit open questions with suggested defaults**: Signal captured 4 open questions (OQ-SIG-001 through OQ-SIG-004) with suggested defaults and "needs answer by" timelines. This enabled Plan to proceed without blocking on human decisions while documenting assumptions clearly.
- **Assumptions documented with impact analysis**: Each assumption included "impact if wrong" which allowed risk-acceptance decisions to be made systematically in Gate.
- **Minimal schema approach**: Choosing name+version only (OQ-SIG-001 suggested default) kept scope tight. ADR explicitly marked extended metadata (git SHA, build date) as out-of-scope but noted JSON is extensible.
- **Coexistence requirement (REQ-004)**: Explicitly stating that --version flag must remain unchanged prevented scope creep and protected existing behavior.

### What Didn't
- **No explicit MUST/SHOULD markers on requirements**: merge_decision.md noted "Priority classification: UNKNOWN - No explicit MUST/SHOULD markers in requirements.md". This made it harder to distinguish critical vs. optional requirements during Gate.
- **Open questions remained OPEN through the run**: All 4 signal OQs and 3 plan OQs remained in OPEN state. While suggested defaults allowed progress, explicit resolution (even "accepted suggested default") would have reduced ambiguity.

### Recommendation
- ACTION: Add MUST/SHOULD/MAY markers to requirements.md template (Flow 1) to enable explicit priority classification in Gate
- ACTION: Add resolution step to Flow 2 or Flow 3 that explicitly closes open questions with "A: accepted suggested default" or "A: <decision>"

### Evidence
- `.runs/cli-version-cmd/signal/open_questions.md`: 4 OQs with suggested defaults, all remained OPEN
- `.runs/cli-version-cmd/gate/merge_decision.md`: "Priority classification: UNKNOWN"

---

## Learning: Design

### What Worked
- **Pattern reuse from existing codebase**: ADR correctly identified time.rs as the minimal subcommand pattern to follow. This reduced design ambiguity and ensured implementation matched existing conventions.
- **Typed struct with derive(Serialize)**: ADR driver DR-002 explicitly chose OPT-001 for compile-time schema enforcement. code_critique.md confirmed no contract violations.
- **Explicit non-goals**: ADR documented what was NOT being done (pack-check version, replacing --version, extended metadata). This prevented scope creep during Build.
- **Assumption inventory with stable markers**: ADR included 6 numbered assumptions (ASM-001 through ASM-006) with impact analysis, enabling systematic tracking.

### What Didn't
- **ADR does not capture environment prerequisites**: The ADR focused on code design but did not document that cargo-tarpaulin would be needed for coverage verification. This gap surfaced in Gate as RSK-006.
- **Risk assessment did not predict credential/permission issues**: RSK-001 through RSK-004 were code/design risks. The actual blocking issue (push permission denied to wrong account) was operational, not captured in risk assessment.

### Recommendation
- ACTION: Add "Environment Prerequisites" section to ADR template that lists tooling required for verification (coverage tools, linters, security scanners)
- ACTION: Add "Operational Risks" category to early_risks.md template covering credentials, permissions, branch protection

### Evidence
- `.runs/cli-version-cmd/plan/adr.md`: Risks RSK-001 through RSK-004 are all code/design risks
- `.runs/cli-version-cmd/gate/coverage_audit.md`: UNVERIFIED due to missing cargo-tarpaulin
- `.runs/cli-version-cmd/gate/git_status.md`: Push failed due to credential mismatch ("danshieldspala" vs "d4nshields")

---

## Learning: Build

### Test Quality
- **TDD approach worked cleanly**: test_changes_summary.md documented "8 passed; 6 failed (expected pre-implementation)". Tests were written first, failed as expected, then passed after implementation.
- **11 version-specific tests added**: Comprehensive coverage of all REQs and most NFRs. test_critique.md confirmed 14/14 tests passing.
- **Clear REQ-to-test mapping**: Coverage table in test_critique.md explicitly mapped each REQ to specific test(s), making verification auditable.
- **Implicit coverage for untestable scenarios**: test_critique.md explicitly noted which BDD scenarios were "not testable" (failure writes to stderr) or "implicitly covered" (determinism covers environment-independence).

### Iteration Patterns
- **Single implementation commit**: All code changes (version.rs, mod.rs, main.rs) landed in one commit (091fa24). Clean separation of test-writing and implementation phases.
- **No rework between critics**: code_critique.md and test_critique.md both returned `can_further_iteration_help: no` on first pass. Implementation matched plan without iteration.
- **Documentation gap caught early**: code_critique.md flagged NFR-COMP-002 (CLAUDE.md update) as minor gap. This was addressed before Gate.

### Recommendation
- ACTION: Add "testability analysis" step to BDD authoring (Flow 1) to explicitly mark scenarios as "directly testable", "implicitly testable", or "not testable by design"
- ACTION: Consider adding TDD checkpoint to Build flow that expects initial test failures before implementation

### Evidence
- `.runs/cli-version-cmd/build/test_changes_summary.md`: "Test Run Results: 8 passed; 6 failed (expected pre-implementation)"
- `.runs/cli-version-cmd/build/code_critique.md`: `can_further_iteration_help: no`, only minor: NFR-COMP-002 doc gap
- `.runs/cli-version-cmd/build/test_critique.md`: 14/14 passed, coverage_summary.bdd_scenarios_covered: 11/16

---

## Assumptions

| Assumption | Held? | Evidence |
|-----------|-------|----------|
| ASM-001: Minimal JSON schema (name + version) is sufficient | Yes | Implementation uses VersionInfo with name/version only; no change requests |
| ASM-002: JSON output is acceptable deviation from scalar contract | Yes | code_critique.md found no contract violations; documented as intentional exception |
| ASM-003: --version flag and version subcommand coexist | Yes | REQ-004 implemented and tested (version_flag_and_subcommand_coexist test passes) |
| ASM-004: pack-check version subcommand is out of scope | Yes | No scope creep; RSK-003 noted for follow-up run |
| ASM-005: Pretty-printed JSON is preferred | Yes | Implementation uses to_string_pretty(); test verifies multiline output |
| ASM-006: Name field should be literal "demoswarm" | Yes | Implementation hardcodes "demoswarm" per assumption |
| Coverage tooling would be available | No | cargo-tarpaulin not installed; RSK-006 created and accepted |
| GitHub credentials would be correctly configured | No | Push denied to "danshieldspala" attempting to push to "d4nshields" repo |
| Branch protection would be enabled on main | No | GitHub API returned 404 "Branch not protected"; verdict NOT_DEPLOYED |

---

## Surprises

- **Credential mismatch during Gate checkpoint**: The git push failed with "Permission to d4nshields/demo-swarm.git denied to danshieldspala". This was not a code issue but an environment configuration issue where the wrong GitHub account credentials were active. The commit succeeded locally; only the push failed.

- **Branch protection check failure in Deploy**: While the merge and release both succeeded (PR #1 merged, tag cli-version-cmd-v1 published), the deployment verdict was NOT_DEPLOYED because branch protection was not enabled on main. The deploy succeeded functionally but failed the governance check. This distinction (functional success vs governance verification) was correctly captured by separating smoke_signal (STABLE) from deployment_verdict (NOT_DEPLOYED).

- **Coverage tooling unavailable**: cargo-tarpaulin and llvm-cov were not installed in the environment. This was correctly handled by creating RSK-006, documenting the gap, and accepting the risk with rationale ("environment gap, not code deficiency").

- **No iteration needed in Build**: Both critics approved on first pass with `can_further_iteration_help: no`. The TDD approach and pattern-following from ADR resulted in clean implementation without rework.

---

## Actions

- ACTION: Add "Environment Prerequisites" section to `.claude/agent/plan/design-architect.md` ADR template, listing tooling required for verification (coverage tools, audit tools, linters)

- ACTION: Add "Operational Risks" category to `.claude/agent/signal/risk-assessor.md` early_risks.md template, covering credentials, permissions, branch protection, CI/CD access

- ACTION: Update requirements.md template in `.claude/agent/signal/requirements-author.md` to require MUST/SHOULD/MAY markers on each requirement for explicit priority classification

- ACTION: Add open question resolution checkpoint to Flow 2 (Plan) cleanup that prompts for explicit "A: accepted suggested default" or "A: <actual decision>" on each OQ

- ACTION: Add "testability" field to BDD scenario template: "testable: direct | implicit | not-testable" with explanation required for non-direct

- ACTION: Document credential verification as pre-flight check for runs (verify `gh auth status` and `git remote -v` match expected account)

- ACTION: Add branch protection check to Flow 5 pre-flight (before attempting merge) to fail fast rather than after merge succeeds

---

## Machine Summary
```yaml
status: VERIFIED
recommended_action: PROCEED
route_to_flow: null
route_to_agent: null
blockers: []
missing_required: []
concerns:
  - Coverage tooling gap (RSK-006) should be addressed for future runs
  - Branch protection should be enabled before future merges
  - Credential configuration should be verified before runs
```
