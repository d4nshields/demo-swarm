# Test Execution Report

## Machine Summary
status: VERIFIED
recommended_action: PROCEED
route_to_flow: null
route_to_agent: null
blockers: []
missing_required: []
concerns: []
test_summary:
  mode: verify
  command: "cargo test --color=always (in tools/demoswarm-runs-tools)"
  exit_code: 0
  passed: 14
  failed: 0
  skipped: 0
  xfailed: null
  xpassed: null
  duration_seconds: null

## Inputs Used
- /home/daniel/work/demo-swarm/.claude/skills/test-runner/SKILL.md
- /home/daniel/work/demo-swarm/tools/demoswarm-runs-tools/Cargo.toml

## Execution
- tool: test-runner
- mode: verify
- command: `cargo test --color=always` (cwd: tools/demoswarm-runs-tools)
- exit_code: 0
- duration: unknown

## Canonical Summary (tool-bound)
- test result: ok. 14 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.01s

## Test Summary (Canonical): passed=14 failed=0 skipped=0 xfailed=null xpassed=null

## Failures (if any)
- None

## Notes
- All 14 tests in the `cli_contract` test suite passed.
- Tests verified: version flag outputs, JSON formatting, field presence, determinism, and contract behavior.
- Unit tests in main.rs reported 0 tests (no inline unit tests defined).
- The test suite covers the `demoswarm-runs-tools` crate as specified in the task scope.
