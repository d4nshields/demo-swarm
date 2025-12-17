Feature: Version Subcommand
  As a pack maintainer or CI pipeline
  I want to run a version subcommand that outputs JSON
  So that I can programmatically introspect the installed demoswarm version

  Background:
    Given the demoswarm CLI is installed and accessible

  @REQ-001 @smoke
  Scenario: Version subcommand executes successfully
    When I run "demoswarm version"
    Then the command exits with code 0
    And output is written to stdout

  @REQ-001
  Scenario: Version subcommand appears in help output
    When I run "demoswarm --help"
    Then the output contains "version" as a listed subcommand

  @REQ-002 @smoke
  Scenario: Version output is valid JSON
    When I run "demoswarm version"
    Then the stdout output is valid JSON
    And the JSON can be parsed by standard tools

  @REQ-002
  Scenario: Version JSON contains required name field
    When I run "demoswarm version"
    Then the JSON output contains a "name" field
    And the "name" field value is "demoswarm"

  @REQ-002
  Scenario: Version JSON contains required version field
    When I run "demoswarm version"
    Then the JSON output contains a "version" field
    And the "version" field value is a valid semver string

  @REQ-002
  Scenario: Version JSON is pretty-printed
    When I run "demoswarm version"
    Then the JSON output spans multiple lines
    And the JSON output contains indentation

  @REQ-003
  Scenario: Version matches Cargo.toml package version
    Given the package version in Cargo.toml is known
    When I run "demoswarm version"
    Then the "version" field matches the Cargo.toml package version

  # Justification: REQ-003 AC-3 specifies no runtime file reads; verified by observing no file access errors
  @REQ-003 @edge
  Scenario: Version is available without external file access
    Given the demoswarm binary exists in isolation
    When I run "demoswarm version" without access to Cargo.toml
    Then the command still succeeds with exit code 0
    And the version information is present in the output
