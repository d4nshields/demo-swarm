# Open Questions (Append-only)

This is an append-only register. New items are added in "Update" blocks. Resolutions are appended as `- A:` lines.

## Stable Marker Contract
- Questions: `^- QID:` then `- Q:`
- Assumptions: `^- Assumption:`
- Resolutions: `^- A:`

## Update: run cli-version-cmd

### Questions That Would Change the Spec

#### Category: Technical

- QID: OQ-SIG-001
  - Q: What fields should the JSON output schema include? [OPEN]
  - Suggested default: Minimal schema with `name` and `version` fields only (e.g., `{"name": "demoswarm", "version": "1.0.1"}`)
  - Impact if different: Extended fields (build_date, git_sha, rust_version, features) require build.rs or compile-time env vars, increasing complexity
  - Needs answer by: Flow 2
  - Evidence: issue_normalized.md -> Constraints / Non-negotiables (unknowns section)

- QID: OQ-SIG-002
  - Q: Is breaking the scalar stdout contract acceptable for the version subcommand? [OPEN]
  - Suggested default: Yes, `version` is a documented exception; JSON output is explicitly requested and serves a different purpose than other subcommands
  - Impact if different: If no, would need to output version as single string and add a separate `--format json` flag pattern like pack-check
  - Needs answer by: Flow 2
  - Evidence: github_research.md -> Decisions / Constraints Extracted -> #2 (scalar stdout contract)

- QID: OQ-SIG-003
  - Q: Should the `version` subcommand coexist with or replace the existing `--version` flag? [OPEN]
  - Suggested default: Coexist - keep `--version` for human-readable plain text, add `version` subcommand for machine-readable JSON
  - Impact if different: Replacing `--version` changes established clap behavior and user expectations
  - Needs answer by: Flow 2
  - Evidence: github_research.md -> Questions / Clarifications Needed -> #3

#### Category: Product

- QID: OQ-SIG-004
  - Q: Should `pack-check` also receive a `version` subcommand for tooling suite consistency? [OPEN]
  - Suggested default: Out of scope for this run; can be a follow-up feature request
  - Impact if different: If in-scope, doubles implementation work and requires coordinating two tool changes
  - Needs answer by: Flow 2
  - Evidence: context_brief.md -> Risks Spotted Early; github_research.md -> Questions / Clarifications Needed -> #2

### Assumptions Made to Proceed

- Assumption: The JSON output format is an intentional deviation from the scalar contract for this specific subcommand.
  - Rationale: The signal explicitly requests JSON output, and the `version` subcommand serves introspection (debugging, CI pipelines) where structured data is more valuable
  - Impact if wrong: May need to add `--format` flag to maintain scalar contract purity
  - Linked question: OQ-SIG-002

- Assumption: Build metadata (git SHA, build timestamp) is not required for initial implementation.
  - Rationale: The signal says "tool version info" without specifying extended metadata; minimal implementation satisfies the request
  - Impact if wrong: Would require build.rs setup and compile-time environment variable handling
  - Linked question: OQ-SIG-001

- Assumption: The `pack-check` tool is out of scope for this feature request.
  - Rationale: The signal specifically mentions "demoswarm version CLI subcommand" without mentioning pack-check
  - Impact if wrong: Would expand scope to include a second tool
  - Linked question: OQ-SIG-004

### Resolutions (if any)

(none yet)

### Machine Summary
```yaml
status: VERIFIED
recommended_action: PROCEED
route_to_flow: 1
route_to_agent: problem-framer
output_path: .runs/cli-version-cmd/signal/open_questions.md
questions_added: 4
assumptions_added: 3
missing_required: []
blockers: []
concerns: []
```
