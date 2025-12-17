# ADR: Add demoswarm version Subcommand with JSON Output

## Status
Swarm-Proposed (run-scoped; pending human review at Flow 2 boundary)

## Context
- Problem: The demoswarm CLI lacks a dedicated subcommand for retrieving version information in a machine-parseable format. While the existing `--version` flag provides human-readable text, automation tooling, CI/CD pipelines, and pack maintainers need structured JSON output to programmatically introspect which version of the tooling is installed or resolved by the shim.
- Constraints:
  - Output MUST be JSON format (explicitly requested)
  - Must follow existing clap derive subcommand pattern used by other demoswarm commands
  - Must integrate with the existing Command enum in src/commands/mod.rs
  - Version string must come from compile-time source (Cargo.toml via env!("CARGO_PKG_VERSION"))
  - Existing `--version` flag behavior must remain unchanged (coexistence, not replacement)
  - Implementation should follow the minimal subcommand pattern (similar to time.rs)
- Non-goals:
  - Changing the pack-check tool (separate scope unless explicitly requested)
  - Replacing or modifying the existing `--version` flag behavior
  - Including extended build metadata (git SHA, build date, Rust version) in initial implementation
  - Modifying the scalar output contract for other commands

## Decision Drivers (bound, machine-countable)
Each driver MUST include a stable marker line, then a short explanation.

- DRIVER: DR-001 req=[REQ-005] nfr=[] option_ref="OPT-001"
  - Why it matters: REQ-005 AC-3 explicitly requires implementation to follow the minimal subcommand pattern comparable to time.rs. Only OPT-001 (Standalone Module with Typed Struct) satisfies this by creating a separate version.rs module.

- DRIVER: DR-002 req=[REQ-002] nfr=[NFR-OPS-001] option_ref="OPT-001"
  - Why it matters: REQ-002 requires valid JSON output with specific fields. A typed struct with #[derive(Serialize)] provides compile-time schema enforcement, reducing the risk of malformed output that NFR-OPS-001 guards against.

- DRIVER: DR-003 req=[REQ-001,REQ-003,REQ-004] nfr=[NFR-PERF-001,NFR-REL-001] option_ref="OPT-001"
  - Why it matters: All three options satisfy these requirements equally, but OPT-001 maintains consistency with the codebase pattern while delivering deterministic, low-latency output.

- DRIVER: DR-004 req=[] nfr=[NFR-COMP-001,NFR-COMP-002] option_ref="OPT-001"
  - Why it matters: A typed struct enables easier unit testing of serialization if needed, and clear module boundaries aid documentation. Integration tests remain the primary verification per NFR-COMP-001.

## Decision
We choose **OPT-001: Standalone Module with Typed Struct**.

### What we are doing
- Creating a new subcommand module at `src/commands/version.rs` following the established pattern of time.rs
- Defining a minimal struct `VersionInfo` with `#[derive(Serialize)]` containing `name` and `version` fields
- Serializing the struct using `serde_json::to_string_pretty()` and printing to stdout
- Adding a `Version` variant to the Command enum in mod.rs wrapping a unit struct `VersionCommand`
- Dispatching to `commands::version::run()` from execute_command in main.rs
- Using `env!("CARGO_PKG_VERSION")` for compile-time version embedding
- Adding integration test verifying JSON structure, field presence, and exit code 0
- Updating CLAUDE.md CLI table with version command entry

### What we are NOT doing
- Modifying the existing `--version` flag behavior (REQ-004 requires coexistence)
- Adding extended build metadata (git SHA, build timestamp, Rust version) to the JSON schema
- Adding pack-check version subcommand (explicitly out of scope per ASM-004)
- Adding --format flag or compact output option (pretty-printed JSON is the single output format)
- Embedding version logic inline in main.rs (violates REQ-005 AC-3)

### Requirements & NFR Traceability
- **Satisfied by this decision**
  - REQ-001: Subcommand exists via `demoswarm version`, exit 0, listed in help
  - REQ-002: JSON via serde; pretty-printed; name and version fields present
  - REQ-003: `env!("CARGO_PKG_VERSION")` at compile-time, no runtime I/O
  - REQ-004: Clap `#[command(version)]` on Cli unchanged; separate subcommand
  - REQ-005: Version variant in Command enum; dispatch in execute_command; separate module like time.rs
  - NFR-PERF-001: No I/O, no network; struct allocation + serialize is microseconds
  - NFR-REL-001: Deterministic; same binary yields identical JSON
  - NFR-OPS-001: anyhow Result propagation; errors to stderr via main error handling
  - NFR-COMP-001: Integration test with assert_cmd verifies JSON structure and exit code
  - NFR-COMP-002: CLAUDE.md table updated with version command
- **Trade-offs / partial support**
  - None; all requirements are fully satisfied by this decision

## Alternatives Considered
- ALT: OPT-002 (Inline Module with Manual JSON Construction) -- Rejected because: While satisfying all functional requirements, it sacrifices compile-time schema enforcement. The json! macro approach risks typos (e.g., "verison" instead of "version") that would only be caught at integration test time, not compile time. For a schema that may expand (git_sha, build_date), a typed struct provides safer extensibility. The marginal code reduction (~5 lines) does not justify losing type safety.

- ALT: OPT-003 (Minimal / Embedded in Main) -- Rejected because: It only partially satisfies REQ-005. AC-3 explicitly requires "minimal subcommand pattern comparable to time.rs", and time.rs is a separate module. Embedding version logic in main.rs breaks the established codebase pattern where all subcommands have dedicated modules (count.rs, index.rs, time.rs, etc.). This sets a poor precedent and makes the version logic harder to locate.

## Consequences

### Positive
- Consistent codebase pattern: version.rs follows the same structure as time.rs and other command modules
- Type-safe schema: #[derive(Serialize)] catches schema errors at compile time
- Extensible: Adding fields (git_sha, build_date) later requires only struct field additions
- Testable: Clear module boundary enables targeted testing if needed beyond integration tests
- Machine-readable: CI/CD, automation tools, and developers can programmatically extract version info
- Zero performance overhead: No I/O, no network; microsecond execution time

### Negative
- One more file to maintain: Adds version.rs (~30 lines) to the codebase
- First structured JSON output: Establishes a precedent that version subcommand outputs JSON while other commands output scalars (documented as intentional exception for introspection)
- Python fallback needs update: runs_tools.py will need version subcommand for parity (tracked in impact_map.json)

## Risks and Mitigations
Use stable markers:

- RISK: RSK-001 [LOW] Schema insufficiency -- The minimal schema (name, version) may not satisfy future introspection needs (git SHA, build timestamp, Rust version). Mitigation: JSON is extensible; add fields later without breaking existing consumers who ignore unknown fields. No breaking change required.

- RISK: RSK-002 [LOW] Scalar contract precedent -- The JSON output deviates from the scalar stdout contract used by other demoswarm commands. Future subcommands might adopt JSON without architectural review. Mitigation: Document this as an intentional exception for introspection commands; the version subcommand serves a fundamentally different purpose (machine-readable metadata) than other commands (operational outputs).

- RISK: RSK-003 [LOW] Incomplete tooling suite consistency -- pack-check lacks a version subcommand while demoswarm gains one. Users may expect both tools to behave similarly. Mitigation: pack-check version subcommand is explicitly out of scope; can be added in a follow-up feature request when needed.

- RISK: RSK-004 [LOW] Test timing flakiness -- NFR-PERF-001 targets <50ms execution, but slow CI runners may exceed this occasionally. Mitigation: Use generous timeout in tests (1 second); treat 50ms as informational guidance rather than hard gate. The version command has no I/O so timing is inherently stable.

## Assumptions Made to Proceed
Use stable markers:

- ASM: ASM-001 Minimal JSON schema (name + version fields only) is sufficient for initial implementation. (impact if wrong: Follow-up work needed to add fields like git_sha, build_date, rust_version; JSON is extensible so no breaking change)

- ASM: ASM-002 JSON output is an acceptable deviation from the scalar stdout contract for this specific subcommand. (impact if wrong: Would need --format flag pattern instead; architectural discussion required)

- ASM: ASM-003 The --version flag and version subcommand coexist by design; different use cases justify both. (impact if wrong: Need explicit decision on replacement vs coexistence)

- ASM: ASM-004 pack-check version subcommand is out of scope for this run. (impact if wrong: Scope expansion; may require separate run)

- ASM: ASM-005 Pretty-printed JSON is preferred over compact. (impact if wrong: Minor change to serialization call)

- ASM: ASM-006 The package name field value should be the literal string "demoswarm" (the CLI binary name), not CARGO_PKG_NAME. (impact if wrong: Would need to use env macro instead of hardcoded string)

## Questions / Clarifications Needed
Use stable markers and include suggested defaults:

- Q: OQ-PLN-003 Should the version struct definition live in version.rs or a shared types module? Suggested default: Define inline in version.rs since it is only used by that command. Impact: Shared types module adds abstraction overhead for a single-use struct.

- Q: OQ-PLN-004 Should test strategy include unit tests for struct serialization? Suggested default: Integration test only using assert_cmd. Impact: Unit tests for struct serialization are redundant if serde works correctly.

- Q: OQ-PLN-005 Should CLAUDE.md include example JSON output? Suggested default: Yes, one-liner example for clarity. Impact: Example aids understanding but adds minor maintenance burden when version changes.

## Next Steps (Flow 2 binding)
- Interface/contracts: `.runs/cli-version-cmd/plan/api_contracts.yaml` + `.runs/cli-version-cmd/plan/schema.md`
- Observability: `.runs/cli-version-cmd/plan/observability_spec.md`
- Tests: `.runs/cli-version-cmd/plan/test_plan.md` (map to BDD + verification_notes if present)
- Work breakdown: `.runs/cli-version-cmd/plan/work_plan.md`

## Pointers
- Options: `.runs/cli-version-cmd/plan/design_options.md`
- Requirements: `.runs/cli-version-cmd/signal/requirements.md`
- Problem statement: `.runs/cli-version-cmd/signal/problem_statement.md`
- Impact: `.runs/cli-version-cmd/plan/impact_map.json`
- Open questions: `.runs/cli-version-cmd/plan/open_questions.md`
- Early risks: `.runs/cli-version-cmd/signal/early_risks.md`

## Inventory (machine countable)
(Only the following prefixed lines; do not rename prefixes)

- ADR_CHOSEN_OPTION: OPT-001
- ADR_DRIVER: DR-001
- ADR_DRIVER: DR-002
- ADR_DRIVER: DR-003
- ADR_DRIVER: DR-004
- ADR_ALT: OPT-002
- ADR_ALT: OPT-003
- ADR_RISK: RSK-001
- ADR_RISK: RSK-002
- ADR_RISK: RSK-003
- ADR_RISK: RSK-004
- ADR_ASM: ASM-001
- ADR_ASM: ASM-002
- ADR_ASM: ASM-003
- ADR_ASM: ASM-004
- ADR_ASM: ASM-005
- ADR_ASM: ASM-006
- ADR_Q: OQ-PLN-003 struct location
- ADR_Q: OQ-PLN-004 test strategy
- ADR_Q: OQ-PLN-005 doc example

## Machine Summary Block

```yaml
## Machine Summary
status: VERIFIED
recommended_action: PROCEED
route_to_flow: null
route_to_agent: null
blockers: []
missing_required: []
concerns:
  - JSON output represents first structured output from demoswarm CLI (documented as intentional exception)
  - Python fallback needs version subcommand for parity (tracked in impact_map.json IMP-006)

chosen_option: OPT-001 Standalone Module with Typed Struct
drivers_total: 4
```
