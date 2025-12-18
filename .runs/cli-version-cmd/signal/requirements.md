# Requirements

## Machine Summary
status: VERIFIED

recommended_action: PROCEED
route_to_agent: null
route_to_flow: null

blockers: []

missing_required: []

concerns:
  - JSON output represents first structured output from demoswarm CLI (deviation from scalar contract)
  - pack-check consistency deferred to future request

## Functional Requirements

### REQ-001: Version Subcommand Existence
The system shall provide a `version` subcommand accessible via `demoswarm version`.
- AC-1: Running `demoswarm version` shall execute the version subcommand without error.
- AC-2: Running `demoswarm version` shall return exit code 0 on success.
- AC-3: The subcommand shall be listed in `demoswarm --help` output alongside other subcommands.

### REQ-002: JSON Version Output
The system shall output version information as valid JSON to stdout when the `version` subcommand is executed.
- AC-1: Output shall be parseable by standard JSON parsers (jq, Python json module, serde_json).
- AC-2: Output shall contain a `name` field with string value `"demoswarm"`.
- AC-3: Output shall contain a `version` field with string value matching Cargo.toml package version.
- AC-4: Output shall be pretty-printed (multi-line with indentation) for human readability.

### REQ-003: Version Source
The system shall derive the version string from compile-time package metadata.
- AC-1: Version shall be sourced from `env!("CARGO_PKG_VERSION")` or equivalent compile-time mechanism.
- AC-2: Version string shall match the `version` field in `tools/demoswarm-runs-tools/Cargo.toml`.
- AC-3: No runtime file reads or external calls shall be required to determine version.

### REQ-004: Coexistence with Version Flag
The system shall maintain the existing `--version` flag behavior unchanged.
- AC-1: Running `demoswarm --version` shall continue to output human-readable plain text version info.
- AC-2: The `--version` flag output format shall remain unmodified from current behavior.
- AC-3: Both `demoswarm --version` and `demoswarm version` shall be independently functional.

### REQ-005: Integration with Command Enum
The system shall integrate the version subcommand following the existing clap derive pattern.
- AC-1: A `Version` variant shall exist in the `Command` enum in `src/commands/mod.rs`.
- AC-2: The `execute_command` function in `main.rs` shall dispatch to version handling.
- AC-3: Implementation shall follow the minimal subcommand pattern (comparable to `time.rs`).

## Non-Functional Requirements

### NFR-PERF-001: Execution Time
The system shall execute the version subcommand with minimal latency.
- MET-1: Execution time shall be under 50ms on standard hardware (verified via manual testing or CI benchmark).
- MET-2: No network calls, file I/O, or expensive computations shall be performed.

### NFR-REL-001: Deterministic Output
The system shall produce deterministic output for the version subcommand.
- MET-1: Multiple invocations with the same binary shall produce identical JSON output (verified in CI test).
- MET-2: Output shall not include timestamps, random values, or environment-dependent content.

### NFR-OPS-001: Error Handling
The system shall handle edge cases gracefully.
- MET-1: If version subcommand fails for any reason, exit code shall be non-zero.
- MET-2: Error messages shall be written to stderr, not stdout.
- MET-3: JSON output shall only appear on stdout on successful execution.

### NFR-COMP-001: Test Coverage
The system shall include automated tests for the version subcommand.
- MET-1: At least one unit or integration test shall verify JSON output structure (verified in CI).
- MET-2: Test shall assert presence of `name` and `version` fields.
- MET-3: Test shall verify exit code 0 on success.

### NFR-COMP-002: Documentation Update
The system shall update CLAUDE.md to document the new subcommand.
- MET-1: CLAUDE.md demoswarm CLI table shall include `version` command entry.
- MET-2: Documentation shall describe the JSON output format.

## Assumptions Made
- **ASM-001**: Minimal JSON schema (name + version fields only) is sufficient for initial implementation. (why: signal did not specify additional fields; JSON is extensible for future additions)
  - Impact if wrong: Follow-up work needed to add fields like git_sha, build_date, rust_version.

- **ASM-002**: JSON output is an acceptable deviation from the scalar stdout contract for this specific subcommand. (why: signal explicitly requests JSON; version introspection serves different purpose than other commands)
  - Impact if wrong: Would need `--format` flag pattern instead; architectural discussion required.

- **ASM-003**: The `--version` flag and `version` subcommand coexist by design. (why: different use cases - human vs machine readable; removing --version breaks clap conventions)
  - Impact if wrong: Need explicit decision on replacement vs coexistence.

- **ASM-004**: pack-check version subcommand is out of scope. (why: signal mentions only demoswarm CLI)
  - Impact if wrong: Scope expansion; may require separate run.

- **ASM-005**: Pretty-printed JSON is preferred over compact. (why: aids debugging and human inspection; consumers can minify)
  - Impact if wrong: Minor change to serialization call.

## Questions for Humans
- Q: Should the JSON output include extended build metadata (git SHA, build timestamp, Rust version)? Suggested default: No, start minimal with `{"name": "demoswarm", "version": "X.Y.Z"}`. Impact if different: Requires build.rs setup and compile-time env var handling; increases complexity.

- Q: Should pack-check also receive a `version` subcommand for tooling suite consistency? Suggested default: No, defer to separate request. Impact if different: Doubles implementation work and requires coordinating two tool changes.

- Q: Should JSON output be pretty-printed or compact? Suggested default: Pretty-printed for readability. Impact if different: Minor implementation difference; consumers can handle either.
