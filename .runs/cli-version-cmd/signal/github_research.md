# GitHub Research for cli-version-cmd

## Search Inputs

Terms derived from run_meta.json and orchestrator signal:

1. **Canonical key / aliases**: `cli-version-cmd` (no GitHub issue number)
2. **Keywords from signal text**:
   - `version` (primary)
   - `CLI` / `demoswarm CLI`
   - `subcommand`
   - `JSON output`
   - `version info`
3. **Component names**:
   - `demoswarm-runs-tools`
   - `pack-check`

Queries executed:
- `gh search issues "version CLI" --repo d4nshields/demo-swarm`
- `gh search issues "version" --repo EffortlessMetrics/demo-swarm`
- `gh search prs "version" --repo d4nshields/demo-swarm`
- `gh search prs "version" --repo EffortlessMetrics/demo-swarm`
- `gh search issues "demoswarm CLI"` and `"JSON output"` variants

## Access & Limitations

- **gh CLI**: Available and authenticated (`d4nshields` account via keyring)
- **Token scopes**: `gist`, `read:org`, `repo`, `workflow` (sufficient for read operations)
- **Origin repo**: `d4nshields/demo-swarm` - **issues disabled**
- **Upstream repo**: `EffortlessMetrics/demo-swarm` - no issues or PRs found
- **Rate limits**: Not encountered
- **Search results**: All searches returned empty results

## Related Issues

| # | Title | State | Relevance |
|---|-------|-------|-----------|
| (none found) | - | - | - |

No related issues exist in either the origin or upstream repositories. Issues are disabled on the fork (`d4nshields/demo-swarm`), and the upstream repo (`EffortlessMetrics/demo-swarm`) has no open or closed issues matching the search terms.

## Related PRs

| # | Title | State | Relevance |
|---|-------|-------|-----------|
| (none found) | - | - | - |

No PRs exist in either repository related to version commands, CLI enhancements, or JSON output formats.

## Related Discussions

Not available. The `gh` CLI did not return discussions for either repository. Likely discussions are disabled or empty.

## Decisions / Constraints Extracted

From codebase analysis (no GitHub artifacts available):

1. **Output contract for demoswarm CLI**: The tool uses a "scalar stdout contract" where all helpers print a single scalar (`null`, integer, or string). See `tools/demoswarm-runs-tools/src/output.rs`.

2. **Clap derive pattern**: Both `demoswarm` and `pack-check` use clap 4.x with derive macros. Both already have `#[command(version)]` which enables the `--version` flag.

3. **JSON output pattern in pack-check**: The `pack-check` tool has a `--format json` option that outputs structured JSON via `serde_json::to_string_pretty()`. This establishes a precedent for JSON output in the tooling suite.

4. **Subcommand structure**: Commands are defined in `src/commands/mod.rs` as a clap `Subcommand` enum. Adding a new subcommand requires:
   - New module in `src/commands/`
   - Enum variant in `Command`
   - Match arm in `execute_command()` in `main.rs`

5. **Version source**: Package versions are defined in `Cargo.toml`:
   - `demoswarm-runs-tools`: version `1.0.1`
   - `demoswarm-pack-check`: version `1.0.1`

## Prior Art Pointers (Local Codebase)

| Path | Note |
|------|------|
| `tools/demoswarm-runs-tools/src/commands/mod.rs` | Subcommand enum definition - pattern to follow |
| `tools/demoswarm-runs-tools/src/commands/time.rs` | Simple subcommand example (single `now` operation) |
| `tools/demoswarm-runs-tools/src/output.rs` | Scalar output helpers (`print_scalar`, `print_null`) |
| `tools/demoswarm-pack-check/src/cli.rs` | `OutputFormat` enum (Text/Json) - potential pattern |
| `tools/demoswarm-pack-check/src/reporter.rs` | JSON serialization pattern with `serde_json::to_string_pretty` |
| `tools/demoswarm-runs-tools/Cargo.toml` | Version string location (`1.0.1`) |
| `tools/demoswarm-runs-tools/tests/cli_contract.rs` | Test patterns for CLI behavior |

## Implications for Flow 1

### Constraints for Requirements

1. **JSON output is explicitly requested**: The signal specifies JSON format. This should be a hard requirement.

2. **Follow existing scalar contract vs. structured output**: The demoswarm CLI currently outputs scalars. A `version` subcommand outputting JSON would be the first command to output structured data. Decision needed: break scalar contract or make this a special case.

3. **Version source must be compile-time**: Rust embeds version from `Cargo.toml` via `env!("CARGO_PKG_VERSION")`. No runtime version file needed.

4. **Consistency question**: Should `pack-check` also get a `version` subcommand? Both tools are part of the same suite.

### Risks from Prior Attempts

- No prior attempts found. This is greenfield within the existing architecture.

### Stakeholders Hinted

- Pack maintainers (mentioned in `issue_normalized.md`)
- CI/CD pipelines (automation consumers)
- The demoswarm CLI is invoked via shims (`.claude/scripts/demoswarm.sh`), so version introspection supports debugging shim resolution.

### Do-Not-Repeat Landmines

- None identified (no prior failed attempts).

## Assumptions Made to Proceed

1. **No external GitHub context exists**: Confirmed via exhaustive search. This feature has no prior discussion, rejected PRs, or related issues.

2. **Upstream repo is authoritative but sparse**: The upstream `EffortlessMetrics/demo-swarm` has no issues/PRs; the fork is where active development happens.

3. **JSON output format is intentional deviation**: The request explicitly asks for JSON, even though the CLI currently uses scalar output. This is an accepted design decision for the version subcommand.

## Questions / Clarifications Needed

1. What fields should the JSON output include? Minimal (name, version) vs. extended (build info, Rust version, features)?

2. Should `pack-check` receive the same treatment for consistency?

3. Should the existing `--version` flag behavior change, or should `version` subcommand coexist with it?

## Inventory (machine countable)

- ISSUE: (none) relevance=N/A state=N/A
- PR: (none) relevance=N/A state=N/A
- CODE_REF: tools/demoswarm-runs-tools/src/commands/mod.rs note=subcommand enum definition
- CODE_REF: tools/demoswarm-runs-tools/src/commands/time.rs note=simple subcommand pattern
- CODE_REF: tools/demoswarm-runs-tools/src/output.rs note=output helpers
- CODE_REF: tools/demoswarm-pack-check/src/cli.rs note=OutputFormat enum pattern
- CODE_REF: tools/demoswarm-pack-check/src/reporter.rs note=JSON serialization pattern
- CODE_REF: tools/demoswarm-runs-tools/Cargo.toml note=version string source
- CODE_REF: tools/demoswarm-runs-tools/tests/cli_contract.rs note=test patterns

## Machine Summary

```yaml
status: VERIFIED
recommended_action: PROCEED
route_to_flow: 1
route_to_agent: problem-framer
blockers: []
missing_required: []
concerns:
  - Issues disabled on fork repository (d4nshields/demo-swarm)
  - Upstream repo has no issues or PRs to reference
  - No prior GitHub discussions found for this feature
```
