Feature: Version Subcommand Error Handling
  As a system operator
  I want the version subcommand to handle errors gracefully
  So that I can distinguish success from failure in automation

  Background:
    Given the demoswarm CLI is installed and accessible

  # Justification: NFR-OPS-001 specifies error handling behavior that is testable as BDD
  @REQ-001 @NFR-OPS-001 @error
  Scenario: Successful execution writes to stdout only
    When I run "demoswarm version"
    Then output is written to stdout
    And no output is written to stderr
    And the exit code is 0

  # Justification: REQ-001 governs subcommand execution; NFR-OPS-001 specifies error behavior contract
  @REQ-001 @NFR-OPS-001 @error
  Scenario: Failure writes errors to stderr
    Given a condition that causes version subcommand to fail
    When I run "demoswarm version"
    Then the exit code is non-zero
    And error messages are written to stderr
    And no JSON output appears on stdout

  # Justification: REQ-002 governs JSON output; NFR-REL-001 adds determinism constraint
  @REQ-002 @NFR-REL-001 @smoke
  Scenario: Multiple invocations produce identical output
    When I run "demoswarm version" multiple times with the same binary
    Then all invocations produce identical JSON output

  # Justification: REQ-002 governs JSON output; NFR-REL-001 constrains content to be deterministic
  @REQ-002 @NFR-REL-001 @edge
  Scenario: Output does not contain environment-dependent content
    When I run "demoswarm version"
    Then the JSON output does not contain timestamps
    And the JSON output does not contain random values
    And the JSON output does not contain environment variables
