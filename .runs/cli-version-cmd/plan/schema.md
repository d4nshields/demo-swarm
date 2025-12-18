# Schema: CLI Version Subcommand

## Overview

This run (cli-version-cmd) defines a CLI subcommand, **not** an HTTP API or database schema.

**System Boundary:** The demoswarm CLI tool (`tools/demoswarm-runs-tools/`)

**Interface List:**
1. `demoswarm version` - New subcommand producing JSON output
2. `demoswarm --version` - Existing flag (unchanged, documented for coexistence)

**Contract Type:** CLI subcommand contract defining:
- Command invocation syntax
- JSON output schema
- Exit codes
- Error behavior
- Determinism guarantees

---

## Data Models

### VersionInfo

The JSON object output by `demoswarm version`.

| Field | Type | Required | Constraints | Description |
|-------|------|----------|-------------|-------------|
| `name` | string | Yes | const: "demoswarm" | Name of the CLI tool (hardcoded) |
| `version` | string | Yes | semver pattern | Version from Cargo.toml at compile time |

**Invariants:**
- `name` is always the literal string `"demoswarm"` (per ASM-006)
- `version` matches `^\d+\.\d+\.\d+(-[a-zA-Z0-9.]+)?(\+[a-zA-Z0-9.]+)?$` (semver)
- `version` is sourced from `env!("CARGO_PKG_VERSION")` at compile time
- No additional properties are present (strict schema)

**Example Output:**
```json
{
  "name": "demoswarm",
  "version": "0.1.0"
}
```

**JSON Schema (Draft 2020-12):**
```json
{
  "$schema": "https://json-schema.org/draft/2020-12/schema",
  "$id": "demoswarm-version-info-v1",
  "type": "object",
  "required": ["name", "version"],
  "additionalProperties": false,
  "properties": {
    "name": {
      "type": "string",
      "const": "demoswarm"
    },
    "version": {
      "type": "string",
      "pattern": "^\\d+\\.\\d+\\.\\d+(-[a-zA-Z0-9.]+)?(\\+[a-zA-Z0-9.]+)?$"
    }
  }
}
```

**Rust Struct (implementation reference):**
```rust
#[derive(Serialize)]
struct VersionInfo {
    name: String,      // Always "demoswarm"
    version: String,   // env!("CARGO_PKG_VERSION")
}
```

### Reserved Fields (Future)

The following fields are NOT implemented in this version but are reserved for future extensibility (per RSK-001 mitigation):

| Field | Type | Description |
|-------|------|-------------|
| `git_sha` | string | Git commit SHA at build time |
| `build_date` | string | ISO8601 build timestamp |
| `rust_version` | string | Rust compiler version |

These would be additive changes (non-breaking) per the compatibility policy.

---

## CLI Contract

### Command: `demoswarm version`

| Aspect | Value |
|--------|-------|
| Invocation | `demoswarm version` |
| Output format | JSON (pretty-printed) |
| Output destination | stdout |
| Exit code (success) | 0 |
| Exit code (error) | 1 |
| Error destination | stderr |
| Arguments | None |
| Options | None (--help via clap) |

**Behavior:**
1. Construct `VersionInfo` struct with hardcoded name and compile-time version
2. Serialize to pretty-printed JSON via `serde_json::to_string_pretty()`
3. Print to stdout
4. Exit 0

**Performance Guarantees:**
- Target latency: <50ms
- No file I/O
- No network calls
- No expensive computations
- Version string embedded at compile time

**Determinism Guarantees:**
- Same binary always produces identical JSON output
- No timestamps, random values, or environment-dependent content

### Command: `demoswarm --version` (Existing, Unchanged)

| Aspect | Value |
|--------|-------|
| Invocation | `demoswarm --version` or `demoswarm -V` |
| Output format | Plain text |
| Output destination | stdout |
| Exit code | 0 |

**Example Output:**
```
demoswarm 0.1.0
```

**Coexistence (REQ-004):**
- `--version` and `version` subcommand serve different purposes
- `--version`: Human-readable, plain text (clap convention)
- `version`: Machine-readable, JSON (automation use case)
- Both are independently functional

---

## Exit Codes

| Code | Meaning | Description |
|------|---------|-------------|
| 0 | Success | Valid JSON written to stdout |
| 1 | Error | Unexpected failure (e.g., stdout write failure) |

**Guarantees:**
- Exit 0 implies valid JSON was written to stdout
- Exit 1 implies error message was written to stderr
- stdout is never polluted with error messages (NFR-OPS-001)

---

## Error Handling

| Error Category | Exit Code | Stderr Message | Likelihood |
|----------------|-----------|----------------|------------|
| stdout write failure | 1 | "Error: failed to write version output" | Rare |
| serialization failure | 1 | "Error: failed to serialize version info" | Impossible |

**Rationale:**
- The version command has essentially no failure modes under normal operation
- VersionInfo is trivially serializable (two string fields)
- stdout write failure would only occur in exceptional circumstances (closed fd, disk full if redirected)

---

## Events/Messages

**Not Applicable.** This feature:
- Does not introduce runtime events
- Does not use message passing
- Is a synchronous CLI command with immediate output
- Has no asynchronous behavior

---

## Compatibility and Versioning

### Schema Versioning Strategy

**Strategy:** Additive Only

- Future versions may add new fields to VersionInfo
- Existing fields will never be removed or have their types changed
- Well-behaved consumers should ignore unknown fields

### Breaking Change Policy

| Change Type | Breaking? | Version Bump |
|-------------|-----------|--------------|
| Adding new fields | No | Minor |
| Removing fields | Yes | Major |
| Changing field types | Yes | Major |
| Changing exit codes | Yes | Major |
| Changing output format (JSON structure) | Yes | Major |

### Error Code Taxonomy

| Code Range | Domain | Description |
|------------|--------|-------------|
| 0 | Success | Command completed successfully |
| 1 | Error | Any error condition |

**Note:** Exit codes 2+ are reserved for future use (e.g., distinguishing error types).

---

## Traceability Mapping

### REQ to Interface Element

| Requirement | Interface Element | Constraint/Behavior |
|-------------|-------------------|---------------------|
| REQ-001 | `demoswarm version` command | Subcommand exists, exit 0, in --help |
| REQ-002 | VersionInfo schema | JSON output with name + version fields |
| REQ-003 | version field source | `env!("CARGO_PKG_VERSION")` compile-time |
| REQ-004 | `--version` flag | Unchanged, coexists with subcommand |
| REQ-005 | Command enum integration | Version variant in enum, dispatch in main |

### NFR to Constraint

| NFR | Constraint | Validation |
|-----|------------|------------|
| NFR-PERF-001 | <50ms, no I/O | Timing test, code review |
| NFR-REL-001 | Deterministic output | Determinism test |
| NFR-OPS-001 | Errors to stderr only | Error handling test |
| NFR-COMP-001 | Integration test coverage | CI execution |
| NFR-COMP-002 | CLAUDE.md update | Doc review |

### Field to Acceptance Criteria

| Field | Requirement | Acceptance Criteria |
|-------|-------------|---------------------|
| `name` | REQ-002 | AC-2: value is "demoswarm" |
| `version` | REQ-002 | AC-3: matches Cargo.toml version |

---

## Assumptions Made to Proceed

| ID | Assumption | Impact if Wrong |
|----|------------|-----------------|
| ASM-001 | Minimal schema (name + version) is sufficient | Add fields later; JSON is extensible |
| ASM-002 | JSON output is acceptable deviation from scalar contract | Would need --format flag pattern |
| ASM-003 | --version and version subcommand coexist by design | Need explicit replacement decision |
| ASM-005 | Pretty-printed JSON is preferred | Change to compact serialization |
| ASM-006 | name field is literal "demoswarm", not CARGO_PKG_NAME | Use env macro instead |

---

## Questions / Clarifications Needed

| ID | Question | Suggested Default | Impact |
|----|----------|-------------------|--------|
| OQ-PLN-003 | Should VersionInfo struct live in version.rs or shared types? | Define inline in version.rs | Shared module adds overhead for single-use struct |
| OQ-PLN-004 | Should tests include unit test for serialization? | Integration test only | Unit tests redundant if serde works |
| OQ-PLN-005 | Should CLAUDE.md include example JSON output? | Yes, one-liner | Minor maintenance when version changes |

---

## Implementation Notes (for Build phase)

### Files to Create/Modify

| File | Action | Description |
|------|--------|-------------|
| `src/commands/version.rs` | Create | New module with VersionInfo struct and run() |
| `src/commands/mod.rs` | Modify | Add Version variant to Command enum |
| `src/main.rs` | Modify | Add dispatch case in execute_command |
| `tests/version_test.rs` | Create | Integration test for version subcommand |
| `CLAUDE.md` | Modify | Add version command to CLI table |

### Code Pattern Reference

The implementation follows the pattern of `src/commands/time.rs`:
- Minimal subcommand module
- Unit struct for command (no arguments)
- Simple run() function returning `Result<()>`
- Uses existing error handling via `anyhow`

---

## Inventory (machine countable)

- CLI_COMMAND: demoswarm version
- CLI_COMMAND: demoswarm --version
- SCHEMA: VersionInfo
- EXIT_CODE: 0
- EXIT_CODE: 1
- ENTITY: VersionInfo

---

## Machine Summary

```yaml
## Machine Summary
status: VERIFIED
recommended_action: PROCEED
route_to_flow: null
route_to_agent: null
blockers: []
missing_required: []
concerns:
  - JSON output is first structured output from demoswarm CLI (documented exception per RSK-002)
  - pack-check does not gain a version subcommand (per ASM-004, out of scope)
  - Python fallback needs version subcommand for parity (tracked in impact_map.json)
```
