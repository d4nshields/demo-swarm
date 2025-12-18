# Stakeholders

## Primary
- **Pack Maintainers**: Directly affected. Need machine-readable version output to debug shim resolution (repo-local vs global vs cargo fallback) and verify tool version alignment across environments.
- **CI/CD Pipeline Operators**: Directly affected. Require structured JSON version output for logging, compatibility checks, and artifact metadata in automated contexts.

## Secondary
- **Automation Tooling Authors**: Affected by current text-parsing brittleness. Will benefit from stable JSON schema for version introspection.
- **Developers**: Affected when debugging environment issues. Currently must manually inspect version output; JSON enables scripted verification.

## Consulted
- **Rust/CLI Maintainers**: Implementation guidance needed on clap patterns, existing subcommand conventions, and JSON serialization approach (context_brief.md cites time.rs as pattern template).

## Informed
- **Documentation Consumers**: CLAUDE.md will be updated to document the new `version` subcommand (NFR-COMP-002). They need to know the command exists and produces JSON.
- **pack-check Users**: Informed that pack-check is explicitly out of scope for this run (OQ-SIG-004); follow-up may add similar capability for tooling suite consistency.

## Notes
- **Single codebase boundary**: All changes are internal to `tools/demoswarm-runs-tools/`. No cross-team or external system dependencies.
- **No external integrations**: No auth providers, payment gateways, databases, or message queues are affected. This is a self-contained CLI enhancement.
- **Existing behavior preserved**: The `--version` flag remains unchanged (REQ-004), so existing scripts using that flag will not break.
