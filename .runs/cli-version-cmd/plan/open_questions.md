# Open Questions (Append-only)

This is an append-only register. New items are added in "Update" blocks. Resolutions are appended as `- A:` lines.

## Stable Marker Contract
- Questions: `^- QID:` then `- Q:`
- Assumptions: `^- Assumption:`
- Resolutions: `^- A:`

## Update: run cli-version-cmd

### Signal Questions Carried Forward

The following Signal-phase questions are relevant to design decisions and carry forward with their suggested defaults accepted:

- A: Minimal JSON schema with `name` and `version` fields only is sufficient for initial implementation. Schema is extensible for future additions without breaking consumers. (resolves OQ-SIG-001) [RESOLVED]

- A: JSON output is an acceptable deviation from the scalar stdout contract for the `version` subcommand specifically. This is a deliberate design choice for machine-readable introspection. (resolves OQ-SIG-002) [RESOLVED]

- A: The `version` subcommand and `--version` flag will coexist. Different use cases (machine-readable JSON vs human-readable plain text) justify both. (resolves OQ-SIG-003) [RESOLVED]

- A: `pack-check` version subcommand is out of scope for this run. Can be addressed in a follow-up feature request. (resolves OQ-SIG-004) [RESOLVED]

### Questions That Would Change the Spec

#### Category: Technical

- QID: OQ-PLN-001
  - Q: Should the version subcommand implementation follow the existing `time.rs` pattern (standalone command module) or be embedded in `main.rs`? [OPEN]
  - Suggested default: Standalone module at `src/commands/version.rs` following the `time.rs` pattern for consistency
  - Impact if different: Embedded in main.rs reduces file count but breaks the established pattern; may complicate future command additions
  - Needs answer by: Flow 3 (Build)
  - Evidence: requirements.md -> REQ-005 AC-3 (minimal subcommand pattern comparable to time.rs)

- QID: OQ-PLN-002
  - Q: Should the JSON output use a struct with serde derive or be constructed manually with serde_json::json! macro? [OPEN]
  - Suggested default: Use a simple struct with `#[derive(Serialize)]` for type safety and future extensibility
  - Impact if different: Manual json! macro is simpler for two fields but loses compile-time type checking and is harder to extend
  - Needs answer by: Flow 3 (Build)
  - Evidence: requirements.md -> REQ-002 (JSON output); problem_statement.md -> Known Context (serde_json precedent in pack-check)

- QID: OQ-PLN-003
  - Q: Should the version struct definition live in the version.rs command module or in a shared types module? [OPEN]
  - Suggested default: Define inline in `version.rs` since it is only used by that command; extract to shared types only if reuse emerges
  - Impact if different: Shared types module adds abstraction overhead for a single-use struct; inline keeps related code together
  - Needs answer by: Flow 3 (Build)
  - Evidence: requirements.md -> NFR-PERF-001 (minimal implementation), REQ-005 (minimal subcommand pattern)

- QID: OQ-PLN-004
  - Q: What should the test strategy be - unit test the struct serialization, integration test the CLI, or both? [OPEN]
  - Suggested default: Integration test using `assert_cmd` to verify end-to-end behavior (JSON output, exit code, field presence) - matches existing test patterns
  - Impact if different: Unit tests for struct serialization are redundant if serde works; integration test validates the full path
  - Needs answer by: Flow 3 (Build)
  - Evidence: requirements.md -> NFR-COMP-001 (test coverage requirements), existing tests in tools/demoswarm-runs-tools/tests/

#### Category: Ops

- QID: OQ-PLN-005
  - Q: Should the CLAUDE.md documentation update include an example JSON output or just describe the command? [OPEN]
  - Suggested default: Include example JSON output in the table entry for clarity (one-liner: `{"name": "demoswarm", "version": "X.Y.Z"}`)
  - Impact if different: No example requires users to run the command to see the format; example aids understanding but adds maintenance burden when version changes
  - Needs answer by: Flow 3 (Build)
  - Evidence: requirements.md -> NFR-COMP-002 (documentation update), CLAUDE.md demoswarm CLI table format

### Assumptions Made to Proceed

- Assumption: The existing test infrastructure (assert_cmd, predicates crates) is sufficient for integration testing the version subcommand.
  - Rationale: These crates are already dependencies of the project and are used for testing other commands.
  - Impact if wrong: May need to add test dependencies or use a different testing approach.
  - Linked question: OQ-PLN-004

- Assumption: Pretty-printed JSON output (via `serde_json::to_string_pretty`) is preferred over compact JSON.
  - Rationale: Signal requirements.md ASM-005 states pretty-printed for readability; consumers can minify if needed.
  - Impact if wrong: Minor change to serialization call if compact is preferred.
  - Linked question: null

- Assumption: The package name field value should be the literal string "demoswarm" (the CLI binary name), not the Cargo package name "demoswarm-runs-tools".
  - Rationale: Users interact with the binary as `demoswarm`; the internal package name is an implementation detail.
  - Impact if wrong: Would need to use `CARGO_PKG_NAME` instead of hardcoded string.
  - Linked question: null

- Assumption: No additional clap attributes beyond basic subcommand definition are needed (no arguments, no flags).
  - Rationale: The version subcommand has no user-configurable options; output format is fixed as JSON.
  - Impact if wrong: Would need to add --format flag or similar if flexibility is required later.
  - Linked question: null

### Resolutions (if any)

(Signal questions resolved above)

### Machine Summary
```yaml
status: VERIFIED
recommended_action: PROCEED
route_to_flow: 2
route_to_agent: null
output_path: .runs/cli-version-cmd/plan/open_questions.md
questions_added: 5
assumptions_added: 4
missing_required: []
blockers: []
concerns:
  - All Plan questions have sensible defaults and do not block design work
  - Four Signal questions resolved by accepting their suggested defaults
  - Implementation is straightforward; most questions are about code organization rather than functionality
```
