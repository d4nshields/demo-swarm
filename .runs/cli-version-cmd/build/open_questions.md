# Open Questions (Append-only)

This is an append-only register. New items are added in "Update" blocks. Resolutions are appended as `- A:` lines.

## Stable Marker Contract
- Questions: `^- QID:` then `- Q:`
- Assumptions: `^- Assumption:`
- Resolutions: `^- A:`

## Update: run cli-version-cmd

### Questions That Would Change the Spec

#### Category: Technical

- QID: OQ-BUILD-001
  - Q: The `time.rs` module uses a nested subcommand pattern (`TimeCommand` wrapping `TimeSubcommand::Now`), while the `version` subcommand requires no arguments. Should `version` use a unit struct pattern (`VersionCommand` with no subcommand enum) or mirror the `time.rs` nested pattern for consistency? [OPEN]
  - Suggested default: Use a unit struct pattern (`#[derive(Args)] pub struct VersionCommand;`) since version has no sub-actions. This is simpler than time.rs pattern and matches the ADR decision for "minimal subcommand".
  - Impact if different: If nested pattern required, would add unnecessary complexity for a single-action command. However, using the nested pattern would provide consistency if future version sub-actions are anticipated (e.g., `version build-info`).
  - Needs answer by: Flow 3 (Build)
  - Evidence: work_plan.md -> ST-002 mentions "Version(version::VersionCommand) variant"; time.rs uses nested TimeCommand/TimeSubcommand pattern

- QID: OQ-BUILD-002
  - Q: The `time.rs` implementation uses `print_scalar()` for output (scalar contract). The version command outputs JSON which deviates from this. Should the version command use `println!()` directly or introduce a new `print_json()` helper for consistency? [OPEN]
  - Suggested default: Use `println!("{}", serde_json::to_string_pretty(&info)?)` directly in version.rs. A `print_json()` helper would be premature since version is the only JSON-emitting command currently.
  - Impact if different: Introducing `print_json()` adds abstraction but would need to be justified by more JSON-emitting commands. If more introspection commands are added later, the helper could be extracted then.
  - Needs answer by: Flow 3 (Build)
  - Evidence: api_contracts.yaml -> version_subcommand.output.format: json; adr.md -> ASM-002 (JSON deviation is intentional)

- QID: OQ-BUILD-003
  - Q: Should the `VersionInfo` struct fields use `String` type or `&'static str` for the hardcoded `name` field, given that `"demoswarm"` is a compile-time constant? [OPEN]
  - Suggested default: Use `String` for both fields for consistency and simpler serde derive. The `name` field can use `"demoswarm".to_string()` and `version` field uses `env!("CARGO_PKG_VERSION").to_string()`.
  - Impact if different: Using `&'static str` would require lifetime annotations on the struct and complicate serde serialization for marginal performance gain on a single-invocation command.
  - Needs answer by: Flow 3 (Build)
  - Evidence: api_contracts.yaml -> schemas.VersionInfo.properties.name.type: string; work_plan.md -> ST-001 acceptance criteria

- QID: OQ-BUILD-004
  - Q: The integration test needs to verify the JSON `version` field value. Should the test use a regex pattern match (`^\d+\.\d+\.\d+`) or extract and parse the actual Cargo.toml version for exact equality? [OPEN]
  - Suggested default: Use regex pattern match for semver format. This is more resilient to version bumps and matches the api_contracts.yaml pattern definition. Exact equality would require test to know the current version at compile time.
  - Impact if different: Exact equality testing would require the test to use `env!("CARGO_PKG_VERSION")` or read Cargo.toml, adding complexity for marginal additional verification.
  - Needs answer by: Flow 3 (Build)
  - Evidence: api_contracts.yaml -> schemas.VersionInfo.properties.version.pattern; work_plan.md -> ST-004 acceptance criteria

#### Category: Ops

- QID: OQ-BUILD-005
  - Q: The work_plan.md lists ST-006 (Python fallback update) as optional. Should Build phase implement this or defer to a follow-up task? The Python version string cannot use Cargo macros and would need hardcoding or a separate version source. [OPEN]
  - Suggested default: Defer ST-006 to follow-up. The Python fallback is a secondary code path, and synchronizing versions between Rust and Python introduces maintenance burden. Document this as a known gap.
  - Impact if different: If implemented now, would need to either hardcode version (drift risk) or create a shared version file read by both Rust build.rs and Python.
  - Needs answer by: Before merge
  - Evidence: work_plan.md -> ST-006 marked "Optional"; adr.md -> Consequences -> Negative mentions Python fallback needs update

### Assumptions Made to Proceed

- Assumption: The version subcommand will use a simple unit struct pattern (`VersionCommand;`) without nested subcommands, unlike `time.rs` which has `TimeSubcommand::Now`.
  - Rationale: The version command has a single action (print JSON); no sub-actions are defined in requirements or contracts.
  - Impact if wrong: Minor refactor to add nested enum if future version sub-actions are needed.
  - Linked question: OQ-BUILD-001

- Assumption: The `anyhow` error handling pattern from other commands is sufficient; no custom error types needed for version command.
  - Rationale: Version command has minimal failure modes (only stdout write failure). The existing anyhow Result pattern handles this adequately.
  - Impact if wrong: Would need to add version-specific error variants if detailed error categorization is required.
  - Linked question: null

- Assumption: The integration test file `tests/cli_contract.rs` is the correct location for the version subcommand test, following the existing test organization.
  - Rationale: Other CLI contract tests are in this file; maintaining test co-location aids discoverability.
  - Impact if wrong: May need to create separate test file or module for version-specific tests.
  - Linked question: OQ-BUILD-004

- Assumption: No multicall entry is needed for the version command in the `MULTICALL_MAP` table.
  - Rationale: Version introspection is typically invoked explicitly, not as a symlinked helper. Other introspection commands (like the main `--version` flag) do not have multicall entries.
  - Impact if wrong: Would need to add `("runs_version", "version")` entry if multicall invocation is desired.
  - Linked question: null

- Assumption: The CLAUDE.md documentation update (ST-005) will add the version command to the existing demoswarm CLI command table without creating a new section.
  - Rationale: The table under "### demoswarm" in CLI Tooling Surface section is the canonical location for command documentation.
  - Impact if wrong: May need to create separate "Introspection Commands" subsection if architectural distinction is desired.
  - Linked question: null

### Resolutions (if any)

- A: Plan questions OQ-PLN-001 through OQ-PLN-005 have sensible defaults accepted per work_plan.md. Proceeding with standalone module, typed struct, inline definition, integration test only, and including doc example. (resolves OQ-PLN-001, OQ-PLN-002, OQ-PLN-003, OQ-PLN-004, OQ-PLN-005) [RESOLVED]

### Machine Summary
```yaml
status: VERIFIED
recommended_action: PROCEED
route_to_flow: null
route_to_agent: null
output_path: .runs/cli-version-cmd/build/open_questions.md
questions_added: 5
assumptions_added: 5
missing_required: []
blockers: []
concerns:
  - OQ-BUILD-001 (struct pattern) has clear default but could affect future extensibility
  - OQ-BUILD-005 (Python fallback) deferred to follow-up; documented as known gap
  - All Build questions have sensible defaults and do not block implementation
```
