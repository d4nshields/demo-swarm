# Normalized Issue

## Machine Summary
status: VERIFIED
recommended_action: PROCEED
route_to_agent: problem-framer
route_to_flow: 1
blockers: []
missing_required: []
notes:
  - Signal is concise; implicit requirements inferred from codebase conventions
  - JSON output format explicitly requested

## Summary

Request to add a `version` subcommand to the `demoswarm` CLI tool that outputs version information in JSON format. This follows the existing subcommand pattern used by the tool and provides machine-parseable version introspection.

## Signal Type
- request_type: feature
- source_type: other (direct request)
- links:
  - none

## Observed vs Expected
- observed: The `demoswarm` CLI has no dedicated `version` subcommand that outputs structured JSON. While clap provides `--version` flag support, it outputs plain text, not JSON.
- expected: A `demoswarm version` subcommand that prints JSON containing tool version information.

## Impact
- affected_users: Pack maintainers, automation tooling, CI/CD pipelines
- severity: low (enhancement)
- frequency: on-demand (version introspection)
- environment: all

## Components Mentioned
- systems/services:
  - demoswarm CLI (tools/demoswarm-runs-tools)
- endpoints/paths:
  - tools/demoswarm-runs-tools/src/main.rs
  - tools/demoswarm-runs-tools/src/commands/mod.rs
- files/modules:
  - New: tools/demoswarm-runs-tools/src/commands/version.rs (implied)

## Constraints / Non-negotiables
- Output MUST be JSON format
- Must follow existing subcommand pattern (clap derive)
- Must integrate with existing Command enum
- unknowns:
  - Exact JSON schema for version output (minimal: tool name, version string)
  - Whether to include additional metadata (build info, rust version, feature flags)
  - Whether pack-check should also get a similar subcommand

## Evidence (bounded)

Raw signal:
> "Add a demoswarm version CLI subcommand that prints JSON with tool version info"
