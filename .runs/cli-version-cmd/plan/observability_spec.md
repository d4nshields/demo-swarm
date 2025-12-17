# Observability Specification: demoswarm version Subcommand

## Overview

### System Boundary
The `demoswarm version` subcommand is a minimal, self-contained CLI command that outputs JSON version information to stdout. It operates entirely within the CLI process with no external dependencies.

### Critical Paths
1. **Version retrieval**: Compile-time constant lookup (`env!("CARGO_PKG_VERSION")`)
2. **JSON serialization**: Serde serialization of a two-field struct
3. **Output**: Write to stdout

### Environments
- Local development
- CI/CD pipelines (primary consumer)
- Production systems running the CLI

### Observability Philosophy for This Change
This subcommand has **minimal observability requirements** by design:
- No network calls (nothing to trace)
- No file I/O (nothing to monitor for latency or errors)
- No user data (nothing to audit)
- Deterministic output (nothing to sample)
- Sub-millisecond execution (nothing to optimize)

The primary observability is **implicit** through the command's exit code and output format.

---

## Metrics

### Applicability Assessment
**Metrics are NOT applicable** for this change because:
1. The command has no persistent process (CLI exits after single invocation)
2. No meaningful counters exist (each invocation is independent)
3. No gauges needed (no state to track)
4. Execution time is too fast to meaningfully histogram (<1ms typical)

### Recommendation
Do not introduce metrics for this subcommand. The overhead of metric collection would exceed the execution time of the command itself.

If future requirements demand CLI telemetry, consider:
- Opt-in telemetry infrastructure (separate feature)
- CI/CD platform native timing (e.g., GitHub Actions step duration)

---

## Logs

### Event Taxonomy
The version subcommand produces exactly one "event": the JSON output to stdout on success.

| Event | Level | When | Fields |
|-------|-------|------|--------|
| Version output | N/A (stdout, not log) | Successful execution | `name`, `version` |
| Error | ERROR (stderr) | Any failure | Error message |

### Logging Rules for This Change
1. **No structured logging required**: The command is too simple to benefit from structured logs
2. **Errors go to stderr**: Per NFR-OPS-001, any errors write to stderr with non-zero exit
3. **No debug/trace logs**: The code path is trivial; debugging via source inspection is sufficient
4. **No PII exposure risk**: No user data is processed or output

### Required Fields (for errors only)
If the command fails (edge case - should be impossible with current design):
- Error description (string)
- Exit code (non-zero integer)

---

## Traces

### Applicability Assessment
**Distributed tracing is NOT applicable** for this change because:
1. No distributed system interaction (single process, no network)
2. No span hierarchy needed (single synchronous operation)
3. No trace context to propagate (no downstream calls)
4. Execution completes in microseconds (tracing overhead would dominate)

### Span Model
None. No spans defined.

### Propagation
None. No trace context propagation.

### Recommendation
Do not add tracing infrastructure to this subcommand. If the broader CLI gains tracing for other subcommands (e.g., `secrets scan`), the version command should remain untraced as an explicit exception.

---

## SLOs

### SLI Definitions
Despite minimal observability needs, the following SLIs can be derived from requirements:

| SLI | Definition | Source |
|-----|------------|--------|
| Availability | Ratio of successful executions (exit 0) to total executions | Exit code |
| Latency | Time from invocation to output | Process timing |
| Correctness | Ratio of outputs with valid JSON schema | JSON parsing |

### SLO Targets

- SLO: SLO-VERSION-AVAIL target=99.99% window=N/A
  - **Rationale**: The command has no external dependencies. Failures indicate binary corruption or system issues, not service degradation.
  - **Measurement**: Not actively measured; rely on integration test suite
  - **Error budget policy**: N/A (no continuous measurement)

- SLO: SLO-VERSION-LATENCY target=<50ms (p99) window=N/A
  - **Rationale**: NFR-PERF-001 specifies <50ms; real-world execution is <1ms
  - **Measurement**: Manual verification or CI benchmark; see RSK-004 for flakiness concern
  - **Error budget policy**: Treat as informational; do not gate on CI timing variance

- SLO: SLO-VERSION-CORRECTNESS target=100% window=N/A
  - **Rationale**: Deterministic output from compile-time constants; no variable input
  - **Measurement**: Integration test verifies JSON structure
  - **Error budget policy**: Any correctness failure is a bug; immediate fix required

### Assumptions
- **ASM-OBS-001**: SLO targets are informational rather than operationally enforced because the command has no runtime dependencies that could degrade. (impact if wrong: Would need telemetry infrastructure to actively measure)
- **ASM-OBS-002**: The 50ms latency target from NFR-PERF-001 has sufficient margin; actual execution is sub-millisecond. (impact if wrong: May need to investigate binary startup time if threshold exceeded)

---

## Alerts

### Applicability Assessment
**Alerting is NOT applicable** for this change because:
1. No continuous process to monitor (CLI exits after use)
2. No threshold-based alerting meaningful for a single-shot command
3. Integration tests serve as the alerting mechanism (CI failure = alert)

### Alert Definitions
None. Alerting is delegated to the test suite and CI pipeline.

### Failure Detection Strategy
| Failure Mode | Detection Mechanism | Response |
|--------------|---------------------|----------|
| Invalid JSON output | Integration test (NFR-COMP-001) | CI fails; fix code |
| Non-zero exit code | Integration test (REQ-001 AC-2) | CI fails; fix code |
| Performance regression | Manual benchmark or CI timing | Investigate if >50ms |

---

## Dashboards

### Applicability Assessment
**Dashboards are NOT applicable** for this change because:
1. No metrics to graph
2. No time-series data to visualize
3. No operational decisions driven by version command behavior

### Recommendation
Do not create dashboards for this subcommand. If CLI usage tracking becomes a requirement, consider it as a separate feature with opt-in telemetry.

---

## Traceability

### Requirements to Signals Mapping

| Requirement | Signal/Mechanism | Verification |
|-------------|------------------|--------------|
| REQ-001 (subcommand exists) | Exit code 0 | Integration test |
| REQ-002 (JSON output) | stdout content | Integration test JSON parsing |
| REQ-003 (version source) | Compile-time constant | Code review; version string match |
| REQ-004 (coexistence) | Both commands work | Integration test for each |
| REQ-005 (integration) | Dispatch success | Integration test |

### NFRs to Signals Mapping

| NFR | Signal/Mechanism | Verification |
|-----|------------------|--------------|
| NFR-PERF-001 (latency) | Process timing | Manual benchmark; CI timing informational |
| NFR-REL-001 (determinism) | Output comparison | Integration test (multiple runs) |
| NFR-OPS-001 (error handling) | stderr + exit code | Test edge cases if applicable |
| NFR-COMP-001 (test coverage) | CI test suite | Test presence in CI |
| NFR-COMP-002 (documentation) | CLAUDE.md update | Code review |

### Risks to Signals Mapping

| Risk | Monitoring Strategy | Alert/Response |
|------|---------------------|----------------|
| RSK-001 (schema insufficiency) | N/A (future concern) | Extend schema when needed |
| RSK-002 (scalar contract) | Documentation review | Documented as exception |
| RSK-003 (pack-check parity) | N/A (out of scope) | Separate future run |
| RSK-004 (test flakiness) | CI timing variance | Use generous timeout (1s) |

---

## Assumptions Made to Proceed

- **ASM-OBS-001**: SLO targets are informational rather than operationally enforced because the command has no runtime dependencies that could degrade.
- **ASM-OBS-002**: The 50ms latency target from NFR-PERF-001 has sufficient margin; actual execution is sub-millisecond.
- **ASM-OBS-003**: No telemetry infrastructure is required for this change; integration tests provide sufficient observability.
- **ASM-OBS-004**: The minimal observability design is intentional and appropriate for a simple, deterministic CLI command.

---

## Questions / Clarifications Needed

- **OQ-OBS-001**: Should the CLI gain opt-in telemetry infrastructure in the future? Suggested default: No; defer to separate feature request. Impact: Would require significant architecture work for minimal value on this command.

- **OQ-OBS-002**: Should CI report version command timing as a benchmark artifact? Suggested default: No; timing is informational and sub-millisecond. Impact: Would require CI workflow changes; value unclear.

---

## Inventory (machine countable)

- METRIC: none type=N/A labels=[]
- LOG_EVENT: version_error level=error fields=[message]
- TRACE_SPAN: none parent=N/A attrs=[]
- SLO: SLO-VERSION-AVAIL target=99.99% window=N/A
- SLO: SLO-VERSION-LATENCY target=<50ms window=N/A
- SLO: SLO-VERSION-CORRECTNESS target=100% window=N/A
- ALERT: none severity=N/A runbook=N/A

---

## Machine Summary

```yaml
## Machine Summary
status: VERIFIED
recommended_action: PROCEED
route_to_flow: null
route_to_agent: null
blockers: []
missing_required: []
concerns:
  - Observability is minimal by design; this is appropriate for a simple CLI command
  - No runtime metrics/alerts; integration tests serve as the monitoring mechanism
  - SLO targets are informational rather than operationally enforced
```
