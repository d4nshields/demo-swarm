use assert_cmd::{Command, cargo::cargo_bin_cmd};
use serde_json::Value;
use std::io::Write;
use tempfile::NamedTempFile;

fn demoswarm() -> Command {
    cargo_bin_cmd!("demoswarm")
}

// ============================================================================
// Version Subcommand Tests (cli-version-cmd)
// ============================================================================

/// REQ-001: Version subcommand executes successfully with exit code 0
/// Scenario: Version subcommand executes successfully
#[test]
fn version_subcommand_exits_successfully() {
    let mut cmd = demoswarm();
    cmd.arg("version");

    cmd.assert().success().code(0);
}

/// REQ-001: Version subcommand appears in help output
/// Scenario: Version subcommand appears in help output
#[test]
fn version_subcommand_appears_in_help() {
    let mut cmd = demoswarm();
    cmd.arg("--help");

    let output = cmd.output().expect("failed to execute command");
    assert!(output.status.success());

    // Note: clap outputs help to stderr in this CLI configuration
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(
        stderr.contains("version"),
        "--help output should list 'version' subcommand, got: {}",
        stderr
    );
}

/// REQ-002: Version output is valid JSON
/// Scenario: Version output is valid JSON
#[test]
fn version_subcommand_outputs_valid_json() {
    let mut cmd = demoswarm();
    cmd.arg("version");

    let output = cmd.output().expect("failed to execute command");
    assert!(output.status.success(), "command should succeed");

    let stdout = String::from_utf8_lossy(&output.stdout);
    let parsed: Result<Value, _> = serde_json::from_str(&stdout);
    assert!(parsed.is_ok(), "output should be valid JSON: {}", stdout);
}

/// REQ-002: Version JSON contains required name field with value "demoswarm"
/// Scenario: Version JSON contains required name field
#[test]
fn version_json_contains_name_field() {
    let mut cmd = demoswarm();
    cmd.arg("version");

    let output = cmd.output().expect("failed to execute command");
    assert!(output.status.success());

    let stdout = String::from_utf8_lossy(&output.stdout);
    let parsed: Value = serde_json::from_str(&stdout).expect("should be valid JSON");

    assert!(
        parsed.get("name").is_some(),
        "JSON should contain 'name' field"
    );
    assert_eq!(
        parsed["name"].as_str(),
        Some("demoswarm"),
        "name field should be 'demoswarm'"
    );
}

/// REQ-002: Version JSON contains required version field matching semver pattern
/// Scenario: Version JSON contains required version field
#[test]
fn version_json_contains_version_field() {
    let mut cmd = demoswarm();
    cmd.arg("version");

    let output = cmd.output().expect("failed to execute command");
    assert!(output.status.success());

    let stdout = String::from_utf8_lossy(&output.stdout);
    let parsed: Value = serde_json::from_str(&stdout).expect("should be valid JSON");

    assert!(
        parsed.get("version").is_some(),
        "JSON should contain 'version' field"
    );

    let version_str = parsed["version"]
        .as_str()
        .expect("version should be a string");
    // Semver pattern: X.Y.Z with optional prerelease/build metadata
    let semver_regex = regex::Regex::new(r"^\d+\.\d+\.\d+(-[a-zA-Z0-9.]+)?(\+[a-zA-Z0-9.]+)?$")
        .expect("valid regex");
    assert!(
        semver_regex.is_match(version_str),
        "version '{}' should match semver pattern",
        version_str
    );
}

/// REQ-002: Version JSON is pretty-printed (multiline with indentation)
/// Scenario: Version JSON is pretty-printed
#[test]
fn version_json_is_pretty_printed() {
    let mut cmd = demoswarm();
    cmd.arg("version");

    let output = cmd.output().expect("failed to execute command");
    assert!(output.status.success());

    let stdout = String::from_utf8_lossy(&output.stdout);
    // Pretty-printed JSON spans multiple lines
    assert!(
        stdout.lines().count() > 1,
        "pretty-printed JSON should span multiple lines"
    );
    // Pretty-printed JSON contains indentation (spaces at start of lines)
    assert!(
        stdout
            .lines()
            .any(|line| line.starts_with("  ") || line.starts_with("\t")),
        "pretty-printed JSON should contain indentation"
    );
}

/// REQ-003: Version matches Cargo.toml package version
/// Scenario: Version matches Cargo.toml package version
#[test]
fn version_matches_cargo_toml() {
    let mut cmd = demoswarm();
    cmd.arg("version");

    let output = cmd.output().expect("failed to execute command");
    assert!(output.status.success());

    let stdout = String::from_utf8_lossy(&output.stdout);
    let parsed: Value = serde_json::from_str(&stdout).expect("should be valid JSON");
    let json_version = parsed["version"]
        .as_str()
        .expect("version should be a string");

    // Read version from Cargo.toml
    let cargo_toml_path = concat!(env!("CARGO_MANIFEST_DIR"), "/Cargo.toml");
    let cargo_toml_content =
        std::fs::read_to_string(cargo_toml_path).expect("should be able to read Cargo.toml");

    // Extract version from Cargo.toml using simple string parsing
    let version_line = cargo_toml_content
        .lines()
        .find(|line| line.starts_with("version = "))
        .expect("Cargo.toml should contain version line");
    let cargo_version = version_line
        .trim_start_matches("version = ")
        .trim_matches('"')
        .trim();

    assert_eq!(
        json_version, cargo_version,
        "JSON version '{}' should match Cargo.toml version '{}'",
        json_version, cargo_version
    );
}

/// REQ-004: --version flag outputs plain text (not JSON)
/// Scenario: Version flag outputs plain text
#[test]
fn version_flag_outputs_plain_text() {
    let mut cmd = demoswarm();
    cmd.arg("--version");

    let output = cmd.output().expect("failed to execute command");
    assert!(output.status.success());

    // Note: clap outputs --version info to stderr, so check both streams
    let stdout = String::from_utf8_lossy(&output.stdout);
    let stderr = String::from_utf8_lossy(&output.stderr);
    let combined = format!("{}{}", stdout, stderr);

    // Plain text output should NOT be a JSON object (may contain "null" from CLI)
    // But the version string itself (e.g., "demoswarm 1.0.1") should be present
    // Check stderr specifically since clap outputs version there
    let parsed: Result<Value, _> = serde_json::from_str(&stderr);
    let is_json_object = parsed.as_ref().map_or(false, |v| v.is_object());
    assert!(
        !is_json_object,
        "--version output should be plain text, not JSON object"
    );
    // Should contain the tool name (may be in stdout or stderr depending on clap config)
    assert!(
        combined.contains("demoswarm"),
        "--version output should contain tool name, got stdout='{}' stderr='{}'",
        stdout.trim(),
        stderr.trim()
    );
}

/// REQ-004: Both --version flag and version subcommand are independently functional
/// and report matching version numbers
/// Scenario: Version subcommand and flag are independent
#[test]
fn version_flag_and_subcommand_coexist() {
    // Run --version flag
    let mut flag_cmd = demoswarm();
    flag_cmd.arg("--version");
    let flag_output = flag_cmd.output().expect("failed to execute --version");
    assert!(flag_output.status.success(), "--version should succeed");

    // Run version subcommand
    let mut sub_cmd = demoswarm();
    sub_cmd.arg("version");
    let sub_output = sub_cmd.output().expect("failed to execute version");
    assert!(
        sub_output.status.success(),
        "version subcommand should succeed"
    );

    // Extract version from subcommand JSON output
    let sub_stdout = String::from_utf8_lossy(&sub_output.stdout);
    let parsed: Value = serde_json::from_str(&sub_stdout).expect("subcommand should output JSON");
    let json_version = parsed["version"]
        .as_str()
        .expect("JSON should have version");

    // Extract version from flag output (may be stdout or stderr depending on clap config)
    let flag_stdout = String::from_utf8_lossy(&flag_output.stdout);
    let flag_stderr = String::from_utf8_lossy(&flag_output.stderr);
    let flag_combined = format!("{}{}", flag_stdout, flag_stderr);
    assert!(
        flag_combined.contains(json_version),
        "--version output '{}' should contain version '{}' from subcommand",
        flag_combined.trim(),
        json_version
    );
}

/// NFR-REL-001: Multiple invocations produce identical output
/// Scenario: Multiple invocations produce identical output
#[test]
fn version_output_is_deterministic() {
    // First invocation
    let mut cmd1 = demoswarm();
    cmd1.arg("version");
    let output1 = cmd1.output().expect("first invocation failed");
    assert!(output1.status.success());

    // Second invocation
    let mut cmd2 = demoswarm();
    cmd2.arg("version");
    let output2 = cmd2.output().expect("second invocation failed");
    assert!(output2.status.success());

    let stdout1 = String::from_utf8_lossy(&output1.stdout);
    let stdout2 = String::from_utf8_lossy(&output2.stdout);

    assert_eq!(
        stdout1, stdout2,
        "multiple invocations should produce identical output"
    );
}

/// NFR-OPS-001: Successful execution writes to stdout only, no stderr
/// Scenario: Successful execution writes to stdout only
#[test]
fn version_success_writes_stdout_only() {
    let mut cmd = demoswarm();
    cmd.arg("version");

    let output = cmd.output().expect("failed to execute command");
    assert!(output.status.success());

    // stdout should have content
    assert!(
        !output.stdout.is_empty(),
        "stdout should contain version output"
    );

    // stderr should be empty on success
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(
        stderr.is_empty(),
        "stderr should be empty on success, got: '{}'",
        stderr
    );
}

#[test]
fn ms_get_missing_file_returns_null_and_zero_exit() {
    let mut cmd = demoswarm();
    cmd.args([
        "ms",
        "get",
        "--file",
        "./__missing_machine_summary.md",
        "--section",
        "## Machine Summary",
        "--key",
        "status",
        "--null-if-missing",
    ]);

    cmd.assert().success().stdout("null\n");
}

#[test]
fn count_pattern_honors_null_if_zero_flag() {
    let mut tmp = NamedTempFile::new().expect("temp file");
    writeln!(tmp, "no markers present").expect("write");
    tmp.flush().expect("flush temp file");

    let mut cmd = demoswarm();
    cmd.args([
        "count",
        "pattern",
        "--file",
        tmp.path().to_str().expect("path utf8"),
        "--regex",
        "^IMPL_FILE_CHANGED:",
        "--null-if-zero",
    ]);

    cmd.assert().success().stdout("null\n");
}

#[test]
fn invalid_regex_does_not_break_contract() {
    let mut tmp = NamedTempFile::new().expect("temp file");
    writeln!(tmp, "content").expect("write");
    tmp.flush().expect("flush temp file");

    let mut cmd = demoswarm();
    cmd.args([
        "count",
        "pattern",
        "--file",
        tmp.path().to_str().expect("path utf8"),
        "--regex",
        "[",
    ]);

    cmd.assert().success().stdout("null\n");
}
