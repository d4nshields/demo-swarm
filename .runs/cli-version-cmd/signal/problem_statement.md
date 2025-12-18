# Problem Statement

## Machine Summary
status: VERIFIED

recommended_action: PROCEED
route_to_agent: null
route_to_flow: null

blockers: []

missing_required: []

concerns:
  - JSON output breaks current scalar-only stdout contract for demoswarm CLI
  - pack-check tool may need similar treatment for consistency (out of scope unless explicitly requested)

confidence: High

## The Problem

The `demoswarm` CLI tool lacks a dedicated subcommand for retrieving version information in a machine-parseable format. While clap provides a `--version` flag that outputs human-readable text, automation tooling, CI/CD pipelines, and pack maintainers need structured JSON output to programmatically introspect which version of the tooling is installed or resolved by the shim (`demoswarm.sh`).

The absence of this capability means that debugging shim resolution, verifying tool versions in automated contexts, and building version-aware orchestration requires parsing unstructured text output or accessing internal implementation details.

## Who Is Affected

- **Pack maintainers**: Cannot easily verify which version of the demoswarm CLI is active, especially when troubleshooting shim resolution (repo-local vs global vs cargo fallback).
- **CI/CD pipelines**: Cannot programmatically extract version information for logging, compatibility checks, or artifact metadata.
- **Automation tooling**: Any script or tool that needs to introspect the demoswarm version must parse plain text, which is fragile.
- **Developers**: Cannot quickly confirm tool version alignment across environments without manual inspection.

## Constraints

- Output MUST be JSON format (explicitly requested).
- Must follow existing clap derive subcommand pattern used by other demoswarm commands.
- Must integrate with the existing `Command` enum in `src/commands/mod.rs`.
- Version string must come from compile-time source (`Cargo.toml` via `env!("CARGO_PKG_VERSION")`).
- The existing `--version` flag behavior should remain unchanged (coexistence, not replacement).
- Implementation should follow the minimal subcommand pattern (similar to `time.rs`).

## Non-Goals

- Changing the `pack-check` tool (separate scope unless explicitly requested).
- Replacing or modifying the existing `--version` flag behavior.
- Including extended build metadata (git SHA, build date, Rust version, feature flags) unless a default assumption is accepted.
- Modifying the scalar output contract for other commands.

## Success Looks Like

- Running `demoswarm version` prints valid JSON to stdout containing at minimum the tool name and version string.
- The JSON output is parseable by standard tools (jq, Python json module, etc.).
- The command exits with code 0 on success.
- Existing commands and `--version` flag continue to work unchanged.
- Tests exist covering the new subcommand behavior.
- CLAUDE.md documentation is updated to reflect the new subcommand.

## Known Context

- **Primary implementation target**: `tools/demoswarm-runs-tools/src/commands/`
- **Subcommand enum**: `tools/demoswarm-runs-tools/src/commands/mod.rs`
- **Execute dispatch**: `tools/demoswarm-runs-tools/src/main.rs`
- **Simple subcommand template**: `tools/demoswarm-runs-tools/src/commands/time.rs`
- **JSON output precedent**: `tools/demoswarm-pack-check/src/reporter.rs` uses `serde_json::to_string_pretty()`
- **Current version**: `1.0.1` (from `Cargo.toml`)
- **No prior GitHub issues/PRs**: Confirmed via GitHub research; this is greenfield.
- **Scalar output contract**: Current demoswarm commands output scalars; `version` will be the first structured JSON output.

## Assumptions Made to Proceed

- **ASM-1**: Minimal JSON schema (tool name + version) is sufficient for initial implementation.
  - *Why*: The signal did not specify additional fields; minimal is conservative.
  - *If wrong*: Add fields in a follow-up; JSON is extensible without breaking consumers.

- **ASM-2**: The `version` subcommand outputting JSON is an acceptable deviation from the scalar stdout contract.
  - *Why*: The signal explicitly requests JSON; this is a deliberate design choice for this command only.
  - *If wrong*: Need architectural discussion about output format strategy.

- **ASM-3**: The `--version` flag and `version` subcommand should coexist.
  - *Why*: Different use cases (human vs machine); removing --version would break clap conventions.
  - *If wrong*: Would need to decide on replacement vs coexistence explicitly.

- **ASM-4**: `pack-check` version subcommand is out of scope for this run.
  - *Why*: Signal mentions only demoswarm CLI; pack-check was noted as a concern but not requested.
  - *If wrong*: Scope expands; may require separate run or acceptance of larger scope.

## Questions / Clarifications Needed

- Q: Should the JSON output include extended metadata (git SHA, build timestamp, Rust version)? Suggested default: No, start minimal with just `{"name": "demoswarm", "version": "X.Y.Z"}`.

- Q: Should `pack-check` also receive a `version` subcommand for consistency? Suggested default: No, defer to separate request; this run focuses on demoswarm CLI only.

- Q: Should the JSON output be pretty-printed or compact? Suggested default: Pretty-printed for human readability; consumers can minify if needed.
