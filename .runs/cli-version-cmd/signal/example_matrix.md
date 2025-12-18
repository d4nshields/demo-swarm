# Example Matrix

## Machine Summary
status: VERIFIED

recommended_action: PROCEED
route_to_agent: null
route_to_flow: null

blockers: []

missing_required: []

## Coverage Summary

| Requirement | Happy Path | Edge Cases | Error Cases | Scenario Count | Notes |
|-------------|------------|------------|-------------|----------------|-------|
| REQ-001 | Yes | No | Yes | 4 | Subcommand existence, help listing, stdout-only on success, error handling |
| REQ-002 | Yes | Yes | No | 6 | JSON validity, name/version fields, pretty-printing, determinism |
| REQ-003 | Yes | Yes | No | 2 | Cargo.toml match, no external file dependency |
| REQ-004 | Yes | Yes | No | 4 | Plain text flag, unchanged format, independence, non-interference |
| REQ-005 | N/A | N/A | N/A | 0 | Implementation-focused; see verification_notes.md |
| NFR-PERF-001 | N/A | N/A | N/A | 0 | Performance; see verification_notes.md |
| NFR-COMP-001 | N/A | N/A | N/A | 0 | Test coverage meta-requirement; see verification_notes.md |
| NFR-COMP-002 | N/A | N/A | N/A | 0 | Documentation meta-requirement; see verification_notes.md |

## Scenario Index

| REQ | Scenario | Feature File | Tags |
|-----|----------|--------------|------|
| REQ-001 | Version subcommand executes successfully | features/version_subcommand.feature | @REQ-001 @smoke |
| REQ-001 | Version subcommand appears in help output | features/version_subcommand.feature | @REQ-001 |
| REQ-001 | Successful execution writes to stdout only | features/version_error_handling.feature | @REQ-001 @NFR-OPS-001 @error |
| REQ-001 | Failure writes errors to stderr | features/version_error_handling.feature | @REQ-001 @NFR-OPS-001 @error |
| REQ-002 | Version output is valid JSON | features/version_subcommand.feature | @REQ-002 @smoke |
| REQ-002 | Version JSON contains required name field | features/version_subcommand.feature | @REQ-002 |
| REQ-002 | Version JSON contains required version field | features/version_subcommand.feature | @REQ-002 |
| REQ-002 | Version JSON is pretty-printed | features/version_subcommand.feature | @REQ-002 |
| REQ-003 | Version matches Cargo.toml package version | features/version_subcommand.feature | @REQ-003 |
| REQ-003 | Version is available without external file access | features/version_subcommand.feature | @REQ-003 @edge |
| REQ-004 | Version flag outputs plain text | features/version_flag_coexistence.feature | @REQ-004 @smoke |
| REQ-004 | Version flag format remains unchanged | features/version_flag_coexistence.feature | @REQ-004 |
| REQ-004 | Version subcommand and flag are independent | features/version_flag_coexistence.feature | @REQ-004 |
| REQ-004 | Version subcommand does not interfere with flag | features/version_flag_coexistence.feature | @REQ-004 @edge |
| REQ-002 | Multiple invocations produce identical output | features/version_error_handling.feature | @REQ-002 @NFR-REL-001 @smoke |
| REQ-002 | Output does not contain environment-dependent content | features/version_error_handling.feature | @REQ-002 @NFR-REL-001 @edge |

## Gaps

- REQ-005: Implementation-focused requirement (specifies code locations and enum variants). Not expressible as BDD; verified via code review during implementation. See verification_notes.md.
- NFR-PERF-001: Performance requirement (execution time under 50ms). Not expressible as BDD; verified via benchmark or CI timing. See verification_notes.md.
- NFR-COMP-001: Meta-requirement (test coverage existence). Verified by presence of tests in codebase. See verification_notes.md.
- NFR-COMP-002: Meta-requirement (documentation update). Verified by documentation review. See verification_notes.md.

## Notes
- Counts are derived mechanically by signal-cleanup; this matrix is for human navigation.
- NFR-OPS-001 and NFR-REL-001 are behavioral and have BDD scenarios covering their testable aspects.
- The error scenario for NFR-OPS-001 (failure writes to stderr) may be difficult to trigger in practice since version command has no failure modes; included for completeness.
