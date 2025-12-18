# Work Plan for cli-version-cmd

## Machine Summary
status: VERIFIED
recommended_action: PROCEED
route_to_agent: null
route_to_flow: null

blockers: []
missing_required:
  - .runs/cli-version-cmd/plan/test_plan.md (not produced yet; using BDD features from signal)
  - .runs/cli-version-cmd/plan/observability_spec.md (not produced yet; no observability requirements for this feature)

## Scope Snapshot
- **ADR decision**: Create standalone `version.rs` module with `VersionInfo` struct using serde serialization, following the existing `time.rs` pattern.
- **Primary impacts**:
  - IMP-001: New file `src/commands/version.rs` (MEDIUM risk - new code)
  - IMP-002: Modify `src/commands/mod.rs` to add Version variant (MEDIUM risk - enum change)
  - IMP-003: Modify `src/main.rs` to add dispatch arm (MEDIUM risk - control flow change)
  - IMP-004: Modify `tests/cli_contract.rs` to add integration test (LOW risk - test code)
  - IMP-005: Modify `CLAUDE.md` to document command (LOW risk - documentation)
- **Key constraints**:
  - JSON output only (no `--format` flag); pretty-printed
  - Must coexist with `--version` flag (REQ-004)
  - Version from `env!("CARGO_PKG_VERSION")` at compile time (REQ-003)
  - Follow minimal subcommand pattern like `time.rs` (REQ-005)
- **Verification posture**: Integration test via `assert_cmd` verifies JSON structure, field presence, and exit code 0; code review verifies pattern compliance.

## Subtask Index (parseable)

Write this YAML block verbatim to `.runs/cli-version-cmd/plan/subtasks.yaml`:

```yaml
schema_version: subtasks_v1
subtasks:
  - id: ST-001
    title: "Create version.rs command module with VersionInfo struct"
    status: TODO
    depends_on: []
    req_ids: ["REQ-002", "REQ-003", "REQ-005"]
    nfr_ids: ["NFR-PERF-001", "NFR-REL-001"]
    acceptance_criteria:
      - "File exists at tools/demoswarm-runs-tools/src/commands/version.rs"
      - "VersionInfo struct has name field (String) and version field (String)"
      - "Struct derives Serialize from serde"
      - "run() function accepts VersionCommand and returns anyhow::Result<()>"
      - "Version sourced from env!(\"CARGO_PKG_VERSION\") at compile time"
      - "JSON output via serde_json::to_string_pretty()"
      - "Output printed to stdout (not using print_scalar, which is for scalars)"
    scope_hints:
      code_roots: ["tools/demoswarm-runs-tools/src/commands/"]
      test_roots: []
      doc_paths: []
      allow_new_files_under: ["tools/demoswarm-runs-tools/src/commands/"]
    touches: ["tools/demoswarm-runs-tools/src/commands/version.rs"]
    tests: []
    observability: []
    estimate: S

  - id: ST-002
    title: "Add Version variant to Command enum in mod.rs"
    status: TODO
    depends_on: ["ST-001"]
    req_ids: ["REQ-005"]
    nfr_ids: []
    acceptance_criteria:
      - "pub mod version; declaration added to mod.rs"
      - "Version(version::VersionCommand) variant added to Command enum"
      - "Version variant has doc comment /// Print version as JSON"
      - "Code compiles without warnings"
    scope_hints:
      code_roots: ["tools/demoswarm-runs-tools/src/commands/"]
      test_roots: []
      doc_paths: []
      allow_new_files_under: []
    touches: ["tools/demoswarm-runs-tools/src/commands/mod.rs"]
    tests: []
    observability: []
    estimate: S

  - id: ST-003
    title: "Add Command::Version dispatch in main.rs execute_command"
    status: TODO
    depends_on: ["ST-002"]
    req_ids: ["REQ-001", "REQ-005"]
    nfr_ids: []
    acceptance_criteria:
      - "Command::Version(sub) => commands::version::run(sub) match arm added"
      - "demoswarm version executes without error"
      - "demoswarm --help lists version subcommand"
    scope_hints:
      code_roots: ["tools/demoswarm-runs-tools/src/"]
      test_roots: []
      doc_paths: []
      allow_new_files_under: []
    touches: ["tools/demoswarm-runs-tools/src/main.rs"]
    tests: []
    observability: []
    estimate: S

  - id: ST-004
    title: "Add integration test for version subcommand"
    status: TODO
    depends_on: ["ST-003"]
    req_ids: ["REQ-001", "REQ-002"]
    nfr_ids: ["NFR-COMP-001", "NFR-REL-001"]
    acceptance_criteria:
      - "Test verifies exit code 0 on success"
      - "Test verifies stdout contains valid JSON"
      - "Test verifies JSON has name field with value demoswarm"
      - "Test verifies JSON has version field that is non-empty"
      - "Test passes with cargo test"
    scope_hints:
      code_roots: []
      test_roots: ["tools/demoswarm-runs-tools/tests/"]
      doc_paths: []
      allow_new_files_under: []
    touches: ["tools/demoswarm-runs-tools/tests/cli_contract.rs"]
    tests: ["@REQ-001", "@REQ-002", "@NFR-COMP-001"]
    observability: []
    estimate: S

  - id: ST-005
    title: "Update CLAUDE.md demoswarm CLI command table"
    status: TODO
    depends_on: ["ST-003"]
    req_ids: []
    nfr_ids: ["NFR-COMP-002"]
    acceptance_criteria:
      - "version command row added to demoswarm CLI table in CLAUDE.md"
      - "Row describes purpose: Print version as JSON"
      - "Table formatting is consistent with existing rows"
    scope_hints:
      code_roots: []
      test_roots: []
      doc_paths: ["CLAUDE.md"]
      allow_new_files_under: []
    touches: ["CLAUDE.md"]
    tests: []
    observability: []
    estimate: S

  - id: ST-006
    title: "Optional: Update Python fallback for version parity"
    status: TODO
    depends_on: ["ST-003"]
    req_ids: []
    nfr_ids: []
    acceptance_criteria:
      - "runs_tools.py has version subcommand that outputs JSON"
      - "JSON output matches Rust implementation schema (name, version fields)"
      - "Fallback works when Rust binary is not available"
    scope_hints:
      code_roots: [".claude/skills/runs-derive/fallback/"]
      test_roots: []
      doc_paths: []
      allow_new_files_under: []
    touches: [".claude/skills/runs-derive/fallback/runs_tools.py"]
    tests: []
    observability: []
    estimate: S

  - id: ST-007
    title: "Verify coexistence with --version flag"
    status: TODO
    depends_on: ["ST-003"]
    req_ids: ["REQ-004"]
    nfr_ids: []
    acceptance_criteria:
      - "demoswarm --version outputs human-readable text (unchanged)"
      - "demoswarm version outputs JSON (new)"
      - "Both commands work independently"
    scope_hints:
      code_roots: []
      test_roots: []
      doc_paths: []
      allow_new_files_under: []
    touches: []
    tests: ["@REQ-004"]
    observability: []
    estimate: S
```

## Subtasks

### ST-001: Create version.rs command module with VersionInfo struct

* **Objective**: Create the new `version.rs` module implementing the version subcommand with typed JSON output.
* **Status**: TODO
* **Planned touchpoints**: `tools/demoswarm-runs-tools/src/commands/version.rs` (new file)
* **REQ/NFR linkage**: REQ-002 (JSON output), REQ-003 (compile-time version), REQ-005 (subcommand pattern), NFR-PERF-001 (execution time), NFR-REL-001 (deterministic)
* **Acceptance criteria**:
  * File exists at `tools/demoswarm-runs-tools/src/commands/version.rs`
  * `VersionInfo` struct has `name` field (String) and `version` field (String)
  * Struct derives `Serialize` from serde
  * `run()` function accepts `VersionCommand` and returns `anyhow::Result<()>`
  * Version sourced from `env!("CARGO_PKG_VERSION")` at compile time
  * JSON output via `serde_json::to_string_pretty()`
  * Output printed to stdout (using `println!`, not `print_scalar` which is for scalars)
* **Scope hints**:
  * Code roots: `tools/demoswarm-runs-tools/src/commands/`
  * Test roots: none
  * Allow new files under: `tools/demoswarm-runs-tools/src/commands/`
* **Tests**: None (tested via integration test in ST-004)
* **Observability**: None (no observability requirements for this feature)
* **Dependencies**: None
* **Risk / blast radius**: Medium - new code file; isolated to single module
* **Estimate**: S

### ST-002: Add Version variant to Command enum in mod.rs

* **Objective**: Register the version module and add the `Version` variant to the `Command` enum.
* **Status**: TODO
* **Planned touchpoints**: `tools/demoswarm-runs-tools/src/commands/mod.rs`
* **REQ/NFR linkage**: REQ-005 (Version variant in Command enum)
* **Acceptance criteria**:
  * `pub mod version;` declaration added to mod.rs
  * `Version(version::VersionCommand)` variant added to `Command` enum
  * Version variant has doc comment `/// Print version as JSON`
  * Code compiles without warnings
* **Scope hints**:
  * Code roots: `tools/demoswarm-runs-tools/src/commands/`
  * Test roots: none
  * Allow new files under: none
* **Tests**: Compilation success
* **Observability**: None
* **Dependencies**: ST-001
* **Risk / blast radius**: Medium - modifying shared enum; could affect existing command dispatch if done incorrectly
* **Estimate**: S

### ST-003: Add Command::Version dispatch in main.rs execute_command

* **Objective**: Wire the Version variant to the version module's `run()` function.
* **Status**: TODO
* **Planned touchpoints**: `tools/demoswarm-runs-tools/src/main.rs`
* **REQ/NFR linkage**: REQ-001 (subcommand existence), REQ-005 (execute_command dispatch)
* **Acceptance criteria**:
  * `Command::Version(sub) => commands::version::run(sub)` match arm added
  * `demoswarm version` executes without error
  * `demoswarm --help` lists `version` subcommand
* **Scope hints**:
  * Code roots: `tools/demoswarm-runs-tools/src/`
  * Test roots: none
  * Allow new files under: none
* **Tests**: Manual verification; formal test in ST-004
* **Observability**: None
* **Dependencies**: ST-002
* **Risk / blast radius**: Medium - modifying main dispatch logic; isolated change to match arm
* **Estimate**: S

### ST-004: Add integration test for version subcommand

* **Objective**: Add automated test verifying version subcommand behavior per NFR-COMP-001.
* **Status**: TODO
* **Planned touchpoints**: `tools/demoswarm-runs-tools/tests/cli_contract.rs`
* **REQ/NFR linkage**: REQ-001 (exit code 0), REQ-002 (JSON with name/version fields), NFR-COMP-001 (test coverage), NFR-REL-001 (deterministic output)
* **Acceptance criteria**:
  * Test verifies exit code 0 on success
  * Test verifies stdout contains valid JSON
  * Test verifies JSON has `name` field with value `"demoswarm"`
  * Test verifies JSON has `version` field that is non-empty
  * Test passes with `cargo test`
* **Scope hints**:
  * Code roots: none
  * Test roots: `tools/demoswarm-runs-tools/tests/`
  * Allow new files under: none
* **Tests**: `@REQ-001`, `@REQ-002`, `@NFR-COMP-001`
* **Observability**: None
* **Dependencies**: ST-003
* **Risk / blast radius**: Low - test-only change; no production code impact
* **Estimate**: S

### ST-005: Update CLAUDE.md demoswarm CLI command table

* **Objective**: Document the new version command in the CLI reference table.
* **Status**: TODO
* **Planned touchpoints**: `CLAUDE.md`
* **REQ/NFR linkage**: NFR-COMP-002 (documentation update)
* **Acceptance criteria**:
  * `version` command row added to demoswarm CLI table in CLAUDE.md
  * Row describes purpose: "Print version as JSON"
  * Table formatting is consistent with existing rows
* **Scope hints**:
  * Code roots: none
  * Test roots: none
  * Doc paths: `CLAUDE.md`
  * Allow new files under: none
* **Tests**: None
* **Observability**: None
* **Dependencies**: ST-003 (command must exist before documenting)
* **Risk / blast radius**: Low - documentation only
* **Estimate**: S

### ST-006: Optional: Update Python fallback for version parity

* **Objective**: Add version subcommand to Python fallback for environments without Rust binary.
* **Status**: TODO
* **Planned touchpoints**: `.claude/skills/runs-derive/fallback/runs_tools.py`
* **REQ/NFR linkage**: None (inferred from impact_map.json IMP-006)
* **Acceptance criteria**:
  * `runs_tools.py` has version subcommand that outputs JSON
  * JSON output matches Rust implementation schema (`name`, `version` fields)
  * Fallback works when Rust binary is not available
* **Scope hints**:
  * Code roots: `.claude/skills/runs-derive/fallback/`
  * Test roots: none
  * Allow new files under: none
* **Tests**: Manual verification
* **Observability**: None
* **Dependencies**: ST-003 (schema must be finalized)
* **Risk / blast radius**: Low - Python fallback is secondary path
* **Estimate**: S

**Note**: ST-006 is marked optional. The Python fallback version string would need to be hardcoded or read from a config file since Python cannot use Cargo compile-time macros. This may warrant a separate discussion on version synchronization strategy.

### ST-007: Verify coexistence with --version flag

* **Objective**: Confirm that the new `version` subcommand coexists with the existing `--version` flag.
* **Status**: TODO
* **Planned touchpoints**: None (verification only)
* **REQ/NFR linkage**: REQ-004 (coexistence requirement)
* **Acceptance criteria**:
  * `demoswarm --version` outputs human-readable text (unchanged)
  * `demoswarm version` outputs JSON (new)
  * Both commands work independently
* **Scope hints**:
  * Code roots: none
  * Test roots: none
  * Allow new files under: none
* **Tests**: `@REQ-004`
* **Observability**: None
* **Dependencies**: ST-003
* **Risk / blast radius**: Low - verification only; no code changes
* **Estimate**: S

## Dependency Graph

```
ST-001 (version.rs module)
   |
   v
ST-002 (mod.rs enum)
   |
   v
ST-003 (main.rs dispatch)
   |
   +--------+--------+--------+
   |        |        |        |
   v        v        v        v
ST-004   ST-005   ST-006   ST-007
(test)   (docs)   (py)     (verify)
```

## Parallelization Opportunities

After ST-003 completes, the following subtasks can run in parallel:
* ST-004 (integration test)
* ST-005 (documentation)
* ST-006 (Python fallback - optional)
* ST-007 (coexistence verification)

ST-001 through ST-003 must be sequential due to code dependencies.

## Rollout Strategy

* **Phase 0 (pre-merge)**: All code changes + tests + documentation in single PR
  * ST-001 through ST-005 complete
  * Tests passing (`cargo test` in `tools/demoswarm-runs-tools`)
  * Manual verification of `demoswarm version` and `demoswarm --version` coexistence

* **Phase 1 (merge)**: PR merged when:
  * All tests pass
  * Code review approved
  * Documentation updated
  * "Green" means: `cargo test` passes, `cargo clippy` clean, no regressions in existing commands

* **Phase 2 (limited exposure)**: Not applicable - this is a CLI introspection command with no staged rollout needed. Feature is available immediately upon binary rebuild.

* **Phase 3 (full)**: Binary is rebuilt and distributed. Users can run `demoswarm version` to get JSON output.

**Rollout Gates**:
* All existing tests pass (no regressions)
* New version subcommand test passes
* Manual smoke test: `demoswarm version | jq .` succeeds

## Rollback Plan

* **Fast rollback lever**: Revert the PR. The change is additive (new subcommand) with no database migrations or external dependencies.

* **Data/schema notes**: No data or schema changes. No persistent state affected.

* **Irreversible steps**: None. The version subcommand is purely additive.

* **Rollback monitoring**: Not applicable - no runtime signals to monitor. If the subcommand causes issues:
  1. Users report broken JSON output or exit code issues
  2. Revert PR
  3. Rebuild binary

* **Mitigation**: Since this is additive code with no side effects:
  - The existing `--version` flag is unchanged
  - No other commands are affected
  - Worst case: revert one PR

## Assumptions

* **ASM-001**: Minimal JSON schema (`name` + `version` fields only) is sufficient. JSON is extensible for future additions without breaking consumers.
* **ASM-002**: JSON output is an acceptable deviation from the scalar stdout contract for the `version` subcommand specifically. This is intentional for machine-readable introspection.
* **ASM-003**: The `--version` flag and `version` subcommand coexist by design. Different use cases (human-readable vs machine-readable) justify both.
* **ASM-004**: `pack-check` version subcommand is out of scope for this run.
* **ASM-005**: Pretty-printed JSON is preferred over compact JSON for readability.
* **ASM-006**: The `name` field value should be the literal string `"demoswarm"` (the CLI binary name), not `CARGO_PKG_NAME` which is `"demoswarm-runs-tools"`.

## Open Questions

Reference: `.runs/cli-version-cmd/plan/open_questions.md`

All Plan-phase questions (OQ-PLN-001 through OQ-PLN-005) have sensible defaults and do not block work. The ADR decision (OPT-001: Standalone Module with Typed Struct) resolves most implementation questions:

* **OQ-PLN-001** (module location): Standalone `version.rs` per ADR decision
* **OQ-PLN-002** (JSON construction): Typed struct with `#[derive(Serialize)]` per ADR decision
* **OQ-PLN-003** (struct location): Inline in `version.rs` (suggested default accepted)
* **OQ-PLN-004** (test strategy): Integration test only (suggested default accepted)
* **OQ-PLN-005** (doc example): Include example JSON in CLAUDE.md (suggested default accepted)

## Inventory (machine countable)

- SUBTASK: ST-001
- SUBTASK: ST-002
- SUBTASK: ST-003
- SUBTASK: ST-004
- SUBTASK: ST-005
- SUBTASK: ST-006
- SUBTASK: ST-007
- ESTIMATE_S: 7
- ESTIMATE_M: 0
- ESTIMATE_L: 0
- ESTIMATE_XL: 0
- TOTAL_SUBTASKS: 7
