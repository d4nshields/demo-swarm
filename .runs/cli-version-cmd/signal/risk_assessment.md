# Risk Assessment

## Machine Summary
status: VERIFIED

recommended_action: PROCEED
route_to_flow: null
route_to_agent: null

blockers: []

missing_required: []

concerns:
  - JSON output represents first structured output from demoswarm CLI (deviation from scalar contract)
  - Open questions (OQ-SIG-001 through OQ-SIG-004) need resolution by Flow 2

severity_summary:
  critical: 0
  high: 0
  medium: 2
  low: 3

## Context
- flow: signal
- run_id: cli-version-cmd
- inputs_used:
  - .runs/cli-version-cmd/run_meta.json
  - .runs/cli-version-cmd/signal/problem_statement.md
  - .runs/cli-version-cmd/signal/requirements.md
  - .runs/cli-version-cmd/signal/early_risks.md
  - .runs/cli-version-cmd/signal/open_questions.md
  - .runs/cli-version-cmd/signal/verification_notes.md
- prior_risk_assessments_seen:
  - none

## Risk Register

| ID | Category | Severity | Status | Summary | Owner |
|----|----------|----------|--------|---------|-------|
| RSK-001 | DATA | MEDIUM | MITIGATED | Version string could become stale if build process bypasses Cargo.toml | backend |
| RSK-002 | OPS | MEDIUM | MITIGATED | JSON output breaks scalar stdout contract precedent | backend |
| RSK-003 | OPS | LOW | OPEN | pack-check inconsistency (no version subcommand) may cause user confusion | backend |
| RSK-004 | PERFORMANCE | LOW | MITIGATED | Version subcommand could introduce unexpected latency | backend |
| RSK-005 | DATA | LOW | MITIGATED | JSON schema evolution could break downstream consumers | backend |

## Risk Details

### RSK-001: Version string staleness from non-Cargo builds
- Category: DATA
- Severity: MEDIUM
- Status: MITIGATED
- Evidence:
  - `.runs/cli-version-cmd/signal/requirements.md` (REQ-003: version from `env!("CARGO_PKG_VERSION")`)
  - `.runs/cli-version-cmd/signal/problem_statement.md` (Constraints: compile-time source)
- Impact:
  - If the tool is built outside of Cargo (e.g., manual rustc invocation, alternative build system), the compile-time macro would not resolve correctly, leading to incorrect or missing version information.
  - Downstream automation relying on version introspection would receive invalid data.
- Mitigation:
  - REQ-003 mandates compile-time sourcing via `env!("CARGO_PKG_VERSION")` which fails at compile time if not available, preventing silent failures.
  - The existing build infrastructure uses Cargo exclusively (install path documented in CLAUDE.md).
- Verification:
  - Build test: compile the tool and verify the version output matches Cargo.toml.
  - CI test (NFR-COMP-001) will catch mismatches.
- Recommendation:
  - No further action required; existing mitigations are adequate.

### RSK-002: JSON output breaks scalar stdout contract
- Category: OPS
- Severity: MEDIUM
- Status: MITIGATED
- Evidence:
  - `.runs/cli-version-cmd/signal/problem_statement.md` (concern: "JSON output breaks current scalar-only stdout contract")
  - `.runs/cli-version-cmd/signal/requirements.md` (ASM-002: JSON output is acceptable deviation)
  - `.runs/cli-version-cmd/signal/open_questions.md` (OQ-SIG-002: scalar contract exception)
- Impact:
  - Future maintainers or tools might assume all demoswarm commands output scalars.
  - Scripts parsing stdout generically could break if they encounter JSON unexpectedly.
- Mitigation:
  - The `version` subcommand is explicitly designed for machine-readable introspection, distinct from operational commands.
  - Documentation update (NFR-COMP-002) will clearly mark this command as producing JSON.
  - Assumption ASM-002 is documented with impact analysis.
- Verification:
  - CLAUDE.md documentation review at Gate.
  - Explicit mention in command help text that output is JSON.
- Recommendation:
  - Accept as a documented exception. The use case (CI/automation introspection) justifies structured output.

### RSK-003: pack-check version inconsistency
- Category: OPS
- Severity: LOW
- Status: OPEN
- Evidence:
  - `.runs/cli-version-cmd/signal/problem_statement.md` (Non-Goals: pack-check changes out of scope)
  - `.runs/cli-version-cmd/signal/open_questions.md` (OQ-SIG-004: pack-check version subcommand)
  - `.runs/cli-version-cmd/signal/requirements.md` (ASM-004: pack-check out of scope)
- Impact:
  - Users may expect `pack-check version` to work similarly to `demoswarm version`.
  - Tooling suite appears inconsistent; CI pipelines checking both tools need different approaches.
- Mitigation:
  - Explicitly documented as out of scope (ASM-004).
  - Can be addressed in a follow-up run without blocking this implementation.
- Verification:
  - Document in CLAUDE.md that `pack-check` does not have a version subcommand (or plan follow-up).
- Recommendation:
  - Accept for this run. Create follow-up ticket if consistency is desired.

### RSK-004: Version subcommand latency
- Category: PERFORMANCE
- Severity: LOW
- Status: MITIGATED
- Evidence:
  - `.runs/cli-version-cmd/signal/requirements.md` (NFR-PERF-001: under 50ms, no I/O)
  - `.runs/cli-version-cmd/signal/verification_notes.md` (benchmark guidance)
- Impact:
  - If the version subcommand performed unexpected I/O or computation, it could introduce latency in CI pipelines or scripts that invoke it frequently.
- Mitigation:
  - NFR-PERF-001 explicitly prohibits network calls, file I/O, or expensive computations.
  - REQ-003 mandates compile-time version sourcing (no runtime file reads).
  - Implementation pattern follows `time.rs` (simple, no dependencies).
- Verification:
  - Execution time check in CI (`timeout 1s demoswarm version`).
  - Code review to confirm no I/O operations.
- Recommendation:
  - Mitigations are adequate; no further action required.

### RSK-005: JSON schema evolution breaking consumers
- Category: DATA
- Severity: LOW
- Status: MITIGATED
- Evidence:
  - `.runs/cli-version-cmd/signal/requirements.md` (REQ-002: name + version fields; ASM-001: minimal schema)
  - `.runs/cli-version-cmd/signal/open_questions.md` (OQ-SIG-001: schema fields question)
- Impact:
  - If additional fields are added later (git_sha, build_date), existing consumers might fail if they perform strict schema validation.
  - Removal of fields would break consumers expecting them.
- Mitigation:
  - Starting with minimal schema (name + version only) as documented in ASM-001.
  - JSON is inherently extensible; additive changes are non-breaking for well-behaved consumers.
  - Requirements explicitly document that additional fields are a follow-up concern.
- Verification:
  - Test suite validates required fields (NFR-COMP-001).
  - Future schema changes should follow semver guidance.
- Recommendation:
  - No action required for this run. Future additions should be additive only.

## Deltas Since Prior (if any)
- NEW: [RSK-001, RSK-002, RSK-003, RSK-004, RSK-005]
- CHANGED: []
- CLOSED: []

## Recommended Next
- Proceed to Flow 2 (Plan) with current risk profile; no blockers identified.
- Resolve open questions (OQ-SIG-001 through OQ-SIG-004) during Plan phase to finalize schema and scope decisions.
- Ensure NFR-COMP-002 (documentation update) explicitly addresses the JSON output deviation from scalar contract.
- Consider creating a follow-up ticket for pack-check version subcommand if tooling suite consistency is desired.
- During Build (Flow 3), verify compile-time version sourcing works correctly with the install path.
