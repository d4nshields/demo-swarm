Feature: Version Flag Coexistence
  As a demoswarm user
  I want both --version flag and version subcommand to work
  So that I have human-readable and machine-readable options

  Background:
    Given the demoswarm CLI is installed and accessible

  @REQ-004 @smoke
  Scenario: Version flag outputs plain text
    When I run "demoswarm --version"
    Then the output is plain text
    And the output is not JSON format

  @REQ-004
  Scenario: Version flag format remains unchanged
    When I run "demoswarm --version"
    Then the output follows the standard clap version format
    And the output contains the tool name
    And the output contains the version number

  @REQ-004
  Scenario: Version subcommand and flag are independent
    When I run "demoswarm --version"
    And I run "demoswarm version"
    Then both commands succeed with exit code 0
    And the version numbers in both outputs match

  @REQ-004 @edge
  Scenario: Version subcommand does not interfere with flag
    When I run "demoswarm --version"
    Then the output format has not changed from the baseline
    And the behavior is identical to before the subcommand was added
