# Test Plan

## Machine Summary
status: VERIFIED
recommended_action: PROCEED
route_to_agent: null
route_to_flow: null

missing_required: []
blockers: []
concerns:
  - Error scenario "Failure writes errors to stderr" may be difficult to trigger since version command has no obvious failure modes (included for contract completeness)

counts:
  scenarios_total: 16
  requirements_total: 9
  requirements_with_scenarios: 6

severity_summary:
  critical: 0
  major: 0
  minor: 1

## Scope

**What this plan covers:**
- Integration tests for the `demoswarm version` subcommand (primary focus)
- Contract tests for JSON output schema validation
- Coexistence tests for `--version` flag
- Determinism verification tests
- Exit code and stdout/stderr contract tests

**What this plan does NOT cover:**
- Unit tests for struct serialization (per OQ-PLN-004, integration tests are sufficient)
- Performance benchmarking (NFR-PERF-001 is verified by code inspection + CI timeout; no hard gate)
- pack-check version subcommand (out of scope per ASM-004)
- Python fallback parity (deferred; tracked in impact_map.json IMP-006)

## Coverage Thresholds

Stable markers (required for coverage-enforcer to parse mechanically):
- COVERAGE_LINE_REQUIRED: 80
- COVERAGE_BRANCH_REQUIRED: 70
- COVERAGE_CRITICAL_PATH: tools/demoswarm-runs-tools/src/commands/version.rs

Additional notes:
- measurement_notes: Use cargo test --workspace with coverage tooling (e.g., cargo-tarpaulin or llvm-cov). Parse summary from test-runner output.
- The version.rs module is small (~30 lines) and should achieve near-100% coverage via integration tests.
- Critical path coverage (90%) applies to version.rs since it is the sole new module.

## Mutation Testing

- mutation_required: false
- mutation_threshold: null
- mutation_scope:
  - tools/demoswarm-runs-tools/src/commands/version.rs
- mutation_tool_hint: cargo-mutants
- rationale: This is a simple, low-risk feature with minimal branching logic. The version subcommand performs no computation beyond struct construction and serialization. Integration tests provide sufficient confidence. Mutation testing is NOT required because: (1) no P0 security/auth/payment code, (2) no complex conditionals, (3) implementation is straightforward serialize-and-print. If desired for validation, mutation testing can be run but should not gate merge.

## Scenario to Test Type Matrix

| REQ | Feature File | Scenario | Priority | Unit | Integration | Contract | E2E | Fuzz | Perf/Obs | Notes |
|-----|--------------|----------|----------|------|-------------|----------|-----|------|----------|-------|
| REQ-001 | version_subcommand.feature | Version subcommand executes successfully | P1 | - | X | - | - | - | - | @smoke; verify exit 0 and stdout |
| REQ-001 | version_subcommand.feature | Version subcommand appears in help output | P2 | - | X | - | - | - | - | Assert "version" in --help |
| REQ-001 | version_error_handling.feature | Successful execution writes to stdout only | P1 | - | X | - | - | - | - | @error; stdout/stderr separation |
| REQ-001 | version_error_handling.feature | Failure writes errors to stderr | P2 | - | X | - | - | - | - | @error; may be mock/skip (see notes) |
| REQ-002 | version_subcommand.feature | Version output is valid JSON | P1 | - | X | X | - | - | - | @smoke; parse with serde_json |
| REQ-002 | version_subcommand.feature | Version JSON contains required name field | P1 | - | X | X | - | - | - | Contract: name == "demoswarm" |
| REQ-002 | version_subcommand.feature | Version JSON contains required version field | P1 | - | X | X | - | - | - | Contract: semver format |
| REQ-002 | version_subcommand.feature | Version JSON is pretty-printed | P2 | - | X | - | - | - | - | Assert multiline + indentation |
| REQ-002 | version_error_handling.feature | Multiple invocations produce identical output | P1 | - | X | - | - | - | - | @smoke; determinism |
| REQ-002 | version_error_handling.feature | Output does not contain environment-dependent content | P2 | - | X | - | - | - | - | @edge; no timestamps/random |
| REQ-003 | version_subcommand.feature | Version matches Cargo.toml package version | P1 | - | X | X | - | - | - | Read Cargo.toml, compare |
| REQ-003 | version_subcommand.feature | Version is available without external file access | P2 | - | X | - | - | - | - | @edge; run binary in isolation |
| REQ-004 | version_flag_coexistence.feature | Version flag outputs plain text | P1 | - | X | - | - | - | - | @smoke; not JSON |
| REQ-004 | version_flag_coexistence.feature | Version flag format remains unchanged | P2 | - | X | - | - | - | - | Baseline comparison |
| REQ-004 | version_flag_coexistence.feature | Version subcommand and flag are independent | P1 | - | X | - | - | - | - | Both succeed, versions match |
| REQ-004 | version_flag_coexistence.feature | Version subcommand does not interfere with flag | P2 | - | X | - | - | - | - | @edge; regression guard |

## Requirement Coverage Summary

| Requirement | Scenarios | Priority | Required Test Types | Notes |
|-------------|-----------|----------|---------------------|-------|
| REQ-001 | 4 | P1 | Integration | Subcommand existence, help listing, exit codes, stdout/stderr |
| REQ-002 | 6 | P1 | Integration, Contract | JSON validity, schema fields, pretty-print, determinism |
| REQ-003 | 2 | P1 | Integration, Contract | Compile-time version, Cargo.toml match |
| REQ-004 | 4 | P1 | Integration | Coexistence with --version flag |
| REQ-005 | 0 | P1 | Code Review | Implementation-focused; no BDD (per verification_notes.md) |
| NFR-PERF-001 | 0 | P2 | Performance (manual) | CI timeout (1s generous); code review for no I/O |
| NFR-REL-001 | 2 (via REQ-002) | P1 | Integration | Determinism scenarios tagged @NFR-REL-001 |
| NFR-OPS-001 | 2 (via REQ-001) | P1 | Integration | Error handling scenarios tagged @NFR-OPS-001 |
| NFR-COMP-001 | 0 | Meta | Gate audit | Verify test existence in cli_contract.rs |
| NFR-COMP-002 | 0 | Meta | Gate audit | Verify CLAUDE.md table updated |

## Contract Test Plan

No `api_contracts.yaml` exists for this run. However, the JSON output schema is a de facto contract.

**JSON Output Contract (implicit):**
```json
{
  "name": "demoswarm",
  "version": "<semver>"
}
```

**Contract assertions:**
1. Output MUST be valid JSON (parseable by serde_json / jq / Python json)
2. `name` field MUST be present and equal to string `"demoswarm"`
3. `version` field MUST be present and match semver pattern (X.Y.Z)
4. `version` field value MUST equal the value in `tools/demoswarm-runs-tools/Cargo.toml`
5. Output MUST be pretty-printed (multiline with indentation)

**Backwards compatibility:**
- This is a new command; no backwards compatibility concerns for initial implementation
- Future schema additions MUST be additive only (per RSK-005 mitigation)

## Non-Behavioral Verification (from verification_notes.md)

| Requirement | Type | Verification Strategy | When |
|-------------|------|----------------------|------|
| REQ-005 | Code Review | Verify: (1) Version variant in Command enum (src/commands/mod.rs), (2) execute_command dispatches to version handler (main.rs), (3) version.rs follows minimal pattern like time.rs | Build / Gate |
| NFR-PERF-001 | Performance | CI: `timeout 1s demoswarm version`; Code review: no I/O, no network calls | Gate |
| NFR-COMP-001 | Test Presence | Gate audit: test exists in `tools/demoswarm-runs-tools/tests/cli_contract.rs` that exercises version subcommand | Gate |
| NFR-COMP-002 | Documentation | Gate audit: CLAUDE.md demoswarm CLI table includes `version` command with JSON format description | Gate |

## Gaps and Questions

- Q: How to test "Failure writes errors to stderr" scenario when version command has no failure modes? Suggested default: Mark as contractual (documents expected behavior if failure were possible); implement as assertion that stderr is empty on success, or skip with documented justification. Impact: Minor; scenario exists for completeness.

- Q: Should tests read Cargo.toml to validate version field? Suggested default: Yes; the test should parse `tools/demoswarm-runs-tools/Cargo.toml` and compare the `version` field against the JSON output. This is explicitly required by REQ-003 AC-2. Impact: Test complexity is low; provides strong assurance.

- Q: Should baseline for "--version flag format unchanged" be captured? Suggested default: Capture current `demoswarm --version` output as a snapshot baseline before implementing version subcommand, or use pattern matching (contains tool name and semver). Impact: Minor; pattern matching is simpler and less brittle.

## Recommended Next

Flow 3 should implement tests in this order:

1. **Primary integration test** (cli_contract.rs): Add test that runs `demoswarm version`, asserts exit code 0, parses JSON output, validates `name` and `version` fields. This covers the @smoke scenarios for REQ-001 and REQ-002.

2. **Version field validation**: Add assertion that `version` field matches `Cargo.toml` package version. This covers REQ-003.

3. **Coexistence test**: Add test that runs both `demoswarm --version` and `demoswarm version`, asserts both succeed, and validates that version numbers match. This covers REQ-004.

4. **Determinism test**: Run `demoswarm version` twice, assert outputs are identical. This covers NFR-REL-001 scenarios.

5. **Help output test**: Assert `demoswarm --help` output contains "version" as a listed subcommand. This covers REQ-001 help listing.

6. **Pretty-print validation**: Assert JSON output spans multiple lines. This covers REQ-002 pretty-print.

7. **Negative case (optional)**: The "Failure writes errors to stderr" scenario can be noted as contractual and not directly testable without artificial failure injection.

## Test Implementation Hints

**Recommended test structure (cli_contract.rs):**
```rust
#[test]
fn version_subcommand_outputs_valid_json() {
    // 1. Run demoswarm version
    // 2. Assert exit code 0
    // 3. Parse stdout as JSON
    // 4. Assert name == "demoswarm"
    // 5. Assert version matches semver pattern
    // 6. Assert version matches Cargo.toml value
}

#[test]
fn version_flag_coexists_with_subcommand() {
    // 1. Run demoswarm --version
    // 2. Assert exit code 0, output is not JSON
    // 3. Run demoswarm version
    // 4. Assert exit code 0, output is JSON
    // 5. Extract version from both, assert they match
}

#[test]
fn version_output_is_deterministic() {
    // 1. Run demoswarm version twice
    // 2. Assert outputs are identical
}

#[test]
fn version_appears_in_help() {
    // 1. Run demoswarm --help
    // 2. Assert output contains "version"
}
```

**Notes:**
- Use `assert_cmd` crate (already in use per impact_map.json)
- Use `serde_json::from_str` to parse and validate JSON
- Read Cargo.toml version using `toml` crate or simple string parsing

---

## Test Strategist Result
status: VERIFIED
recommended_action: PROCEED
route_to_agent: null
route_to_flow: null
missing_required: []
blockers: []
severity_summary:
  critical: 0
  major: 0
  minor: 1
counts:
  scenarios_total: 16
  requirements_total: 9
  requirements_with_scenarios: 6
