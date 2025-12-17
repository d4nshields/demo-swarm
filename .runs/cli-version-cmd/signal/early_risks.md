# Early Risks

## Risks

- RSK-001 [LOW] [OPS]
  - What: JSON output schema may be insufficient for future introspection needs
  - Trigger: A consumer requires build metadata (git SHA, build timestamp, Rust version) not present in minimal schema
  - Mitigation hint: JSON is extensible; add fields later without breaking existing consumers who ignore unknown fields
  - Evidence: OQ-SIG-001 (open question about schema fields); ASM-002 in requirements.md

- RSK-002 [LOW] [OPS]
  - What: Deviation from scalar stdout contract may set undesired precedent
  - Trigger: Future subcommands adopt JSON output without architectural review
  - Mitigation hint: Document this as an intentional exception for introspection commands; establish explicit policy for output formats
  - Evidence: OQ-SIG-002 (open question about scalar contract); concerns in requirements.md Machine Summary

- RSK-003 [LOW] [OPS]
  - What: Incomplete tooling suite consistency (pack-check lacks version subcommand)
  - Trigger: User expects both tools to have version subcommands; debugging requires checking two different interfaces
  - Mitigation hint: Defer to separate run; pack-check can be updated later for consistency
  - Evidence: OQ-SIG-004 (explicitly out of scope); context_brief.md Risks Spotted Early

- RSK-004 [LOW] [PERFORMANCE]
  - What: Test for version subcommand may flake on slow CI runners if performance assertion is too tight
  - Trigger: CI timing varies; 50ms target in NFR-PERF-001 may be exceeded occasionally
  - Mitigation hint: Use generous timeout (verification_notes.md suggests 1s); treat 50ms as informational rather than hard gate
  - Evidence: NFR-PERF-001; verification_notes.md benchmark guidance

## Risk Summary (derived)
- Critical: 0
- High: 0
- Medium: 0
- Low: 4

## Notes
- No SECURITY, COMPLIANCE, or DATA risks identified. This feature has no external interactions, handles no user data, and requires no elevated permissions.
- All risks are LOW severity because the implementation is self-contained, the JSON schema is extensible, and the feature adds capability without removing existing functionality.
- Risk RSK-002 (scalar contract deviation) was flagged in both problem_statement.md and requirements.md concerns; it is tracked but not blocking since the deviation is intentional and documented.
