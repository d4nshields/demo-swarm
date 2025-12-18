# Design Options

## Requirements Binding

Before proposing options, I enumerate the requirement IDs from `.runs/cli-version-cmd/signal/requirements.md`:

**Functional Requirements (REQ):**
- REQ-001: Version Subcommand Existence
- REQ-002: JSON Version Output
- REQ-003: Version Source (compile-time)
- REQ-004: Coexistence with Version Flag
- REQ-005: Integration with Command Enum

**Non-Functional Requirements (NFR):**
- NFR-PERF-001: Execution Time (<50ms, no I/O)
- NFR-REL-001: Deterministic Output
- NFR-OPS-001: Error Handling
- NFR-COMP-001: Test Coverage
- NFR-COMP-002: Documentation Update

Total: 5 REQs, 5 NFRs

---

## OPT-001: Standalone Module with Typed Struct

### Description

Create a new subcommand module at `src/commands/version.rs` following the established pattern of `time.rs`. The module defines a minimal struct `VersionInfo` with `#[derive(Serialize)]` containing `name` and `version` fields. The struct is serialized using `serde_json::to_string_pretty()` and printed to stdout.

The `Command` enum in `mod.rs` gains a `Version` variant wrapping a unit struct `VersionCommand` (no arguments). The `execute_command` function in `main.rs` dispatches to `commands::version::run()`. This is the most idiomatic Rust approach, maximizing type safety and extensibility.

Data flow:
1. User invokes `demoswarm version`
2. clap parses to `Command::Version(VersionCommand)`
3. `main.rs` dispatches to `version::run()`
4. `run()` constructs `VersionInfo { name: "demoswarm", version: env!("CARGO_PKG_VERSION") }`
5. Serializes via `serde_json::to_string_pretty(&info)?`
6. Prints to stdout, returns `Ok(())`

### Requirements Fit

| Requirement | Fit | Notes |
|-------------|-----|-------|
| REQ-001 | SATISFIED | Subcommand exists via `demoswarm version`, exit 0, listed in help |
| REQ-002 | SATISFIED | JSON via serde; pretty-printed; name and version fields present |
| REQ-003 | SATISFIED | `env!("CARGO_PKG_VERSION")` at compile-time |
| REQ-004 | SATISFIED | Clap `#[command(version)]` on Cli unchanged; separate subcommand |
| REQ-005 | SATISFIED | Version variant in Command enum; dispatch in execute_command |
| NFR-PERF-001 | SATISFIED | No I/O, no network; struct allocation + serialize is microseconds |
| NFR-REL-001 | SATISFIED | Deterministic: same binary yields identical JSON |
| NFR-OPS-001 | SATISFIED | anyhow Result propagation; errors to stderr via main error handling |
| NFR-COMP-001 | SATISFIED | Integration test with assert_cmd verifies JSON structure and exit code |
| NFR-COMP-002 | SATISFIED | CLAUDE.md table updated with `version` command |

### Trade-offs

| Dimension | Impact | Rationale |
|-----------|--------|-----------|
| Structure (coupling, components) | Low | Adds one new module; no coupling to other commands |
| Velocity (time-to-first-change) | Low | Follow existing pattern; ~30 LoC new code |
| Governance (auditability, determinism) | Low | Typed struct ensures schema stability; compile-time version |
| Operability (on-call, monitoring, failure modes) | Low | No failure modes beyond stdout write failure |
| Cost (compute, complexity tax) | Low | Trivial runtime cost; one new module to maintain |

### Reversibility
- Rating: Easy
- Switch cost: Delete `version.rs`, remove enum variant, update mod.rs/main.rs; ~15 min
- Blast radius if wrong: None; this is additive feature with no side effects

### Risks

| Risk | Likelihood | Impact | Mitigation (if chosen) |
|------|------------|--------|------------------------|
| Schema insufficiency (RSK-001) | Low | Low | JSON extensible; add fields later without breaking consumers |
| Scalar contract precedent (RSK-002) | Low | Low | Document as intentional exception for introspection |

### Assumptions
- `serde` and `serde_json` are already dependencies (confirmed in Cargo.toml) -- impact if wrong: add dependencies
- Pretty-printed JSON is acceptable for all consumers -- impact if wrong: add `--compact` flag

### When to Choose This
Choose this when you want maximum type safety, idiomatic Rust, and the easiest path to future extensibility (adding git_sha, build_date, etc. later).

---

## OPT-002: Inline Module with Manual JSON Construction

### Description

Create `src/commands/version.rs` but instead of a typed struct, use `serde_json::json!` macro to construct the JSON inline. This avoids defining a struct and derives, trading type safety for brevity.

```rust
use serde_json::json;

pub fn run(_cmd: VersionCommand) -> Result<()> {
    let output = json!({
        "name": "demoswarm",
        "version": env!("CARGO_PKG_VERSION")
    });
    println!("{}", serde_json::to_string_pretty(&output)?);
    Ok(())
}
```

The module structure, enum variant, and dispatch remain identical to OPT-001. The only difference is how the JSON is constructed.

Data flow: Same as OPT-001, except step 4 uses macro instead of struct.

### Requirements Fit

| Requirement | Fit | Notes |
|-------------|-----|-------|
| REQ-001 | SATISFIED | Subcommand exists via `demoswarm version` |
| REQ-002 | SATISFIED | JSON via serde_json::json! macro; pretty-printed |
| REQ-003 | SATISFIED | `env!("CARGO_PKG_VERSION")` at compile-time |
| REQ-004 | SATISFIED | --version flag unchanged |
| REQ-005 | SATISFIED | Version variant in Command enum |
| NFR-PERF-001 | SATISFIED | No I/O, no network |
| NFR-REL-001 | SATISFIED | Deterministic output |
| NFR-OPS-001 | SATISFIED | Errors propagate via Result |
| NFR-COMP-001 | SATISFIED | Integration test verifies JSON structure |
| NFR-COMP-002 | SATISFIED | CLAUDE.md updated |

### Trade-offs

| Dimension | Impact | Rationale |
|-----------|--------|-----------|
| Structure (coupling, components) | Low | Same module structure as OPT-001 |
| Velocity (time-to-first-change) | Low | Slightly less code (~5 lines shorter) |
| Governance (auditability, determinism) | Medium | No compile-time schema enforcement; typos possible |
| Operability (on-call, monitoring, failure modes) | Low | Same as OPT-001 |
| Cost (compute, complexity tax) | Low | Marginally simpler initially; harder to extend |

### Reversibility
- Rating: Easy
- Switch cost: Same as OPT-001
- Blast radius if wrong: None

### Risks

| Risk | Likelihood | Impact | Mitigation (if chosen) |
|------|------------|--------|------------------------|
| Schema typo (e.g., "verison") | Low | Low | Tests catch at integration level, but not at compile time |
| Harder extensibility | Medium | Low | Refactor to struct later if more fields needed |

### Assumptions
- Two-field schema is final for foreseeable future -- impact if wrong: refactor to struct
- No need for type reuse elsewhere -- impact if wrong: duplicate json! in multiple places

### When to Choose This
Choose this when simplicity and minimal code are paramount, and you are confident the schema will remain trivial (two fields) indefinitely.

---

## OPT-003: Minimal / Embedded in Main (No Separate Module)

### Description

Instead of creating a new module, embed the version logic directly in `main.rs` within the `execute_command` match arm. This is the minimal change approach: add the `Command::Version` variant with no associated data, and handle it inline.

```rust
// In main.rs execute_command:
Command::Version => {
    let output = serde_json::json!({
        "name": "demoswarm",
        "version": env!("CARGO_PKG_VERSION")
    });
    println!("{}", serde_json::to_string_pretty(&output)?);
    Ok(())
}
```

No `version.rs` file is created. The `mod.rs` Command enum gains a unit variant `Version` (no wrapper struct).

Data flow:
1. User invokes `demoswarm version`
2. clap parses to `Command::Version`
3. main.rs handles inline, prints JSON
4. Returns Ok(())

### Requirements Fit

| Requirement | Fit | Notes |
|-------------|-----|-------|
| REQ-001 | SATISFIED | Subcommand exists |
| REQ-002 | SATISFIED | JSON output works |
| REQ-003 | SATISFIED | Compile-time version |
| REQ-004 | SATISFIED | --version unchanged |
| REQ-005 | PARTIAL | Integrates with Command enum but violates AC-3 (minimal subcommand pattern comparable to time.rs) -- time.rs is a separate module |
| NFR-PERF-001 | SATISFIED | No I/O |
| NFR-REL-001 | SATISFIED | Deterministic |
| NFR-OPS-001 | SATISFIED | Error handling via Result |
| NFR-COMP-001 | SATISFIED | Integration test still works |
| NFR-COMP-002 | SATISFIED | CLAUDE.md updated |

### Trade-offs

| Dimension | Impact | Rationale |
|-----------|--------|-----------|
| Structure (coupling, components) | Medium | Breaks established pattern; main.rs gains domain logic |
| Velocity (time-to-first-change) | Low | Fewest lines changed |
| Governance (auditability, determinism) | Medium | Inconsistent with other commands |
| Operability (on-call, monitoring, failure modes) | Low | Same runtime behavior |
| Cost (compute, complexity tax) | Low | Slightly simpler initially; sets bad precedent |

### Reversibility
- Rating: Easy
- Switch cost: Refactor to separate module later (~20 min)
- Blast radius if wrong: None

### Risks

| Risk | Likelihood | Impact | Mitigation (if chosen) |
|------|------------|--------|------------------------|
| Pattern inconsistency | High | Low | Acceptable for trivial command; document exception |
| Harder to locate version logic | Medium | Low | grep will find it in main.rs |

### Assumptions
- Consistency with time.rs pattern is not strictly required -- impact if wrong: REQ-005 AC-3 not fully met
- No future need to share version struct -- impact if wrong: must extract later

### When to Choose This
Choose this only if absolute minimal change is the top priority and pattern consistency is explicitly deprioritized. Not recommended given REQ-005 AC-3.

---

## Comparison Matrix

| Dimension | OPT-001 | OPT-002 | OPT-003 |
|-----------|---------|---------|---------|
| REQ coverage (count) | 5/5 | 5/5 | 4/5 |
| NFR coverage (count) | 5/5 | 5/5 | 5/5 |
| Implementation effort | Low | Low | Low |
| Reversibility | Easy | Easy | Easy |
| Ops burden | Low | Low | Low |
| Primary risk | Schema insufficiency (mitigated) | Typo risk (no compile-time) | Pattern inconsistency (REQ-005 PARTIAL) |

## Suggested Default (non-binding)

suggested_default: OPT-001
confidence: High

Rationale (tie to IDs):
- OPT-001 is the only option that fully satisfies REQ-005 AC-3 (minimal subcommand pattern comparable to time.rs)
- Typed struct provides compile-time schema enforcement, reducing NFR-OPS-001 risk of malformed output
- Extensibility path for future fields (git_sha, build_date) is trivial -- add fields to struct
- Consistent with existing codebase patterns (count.rs, index.rs, etc. all have separate modules)
- Negligible additional complexity vs OPT-002 (~5 extra lines for struct definition)

What would change this:
- If explicit mandate to minimize file count, prefer OPT-003
- If schema is guaranteed frozen at two fields forever and team prefers macro style, prefer OPT-002

## Open Questions Affecting Choice

- Q: Should the version struct be reusable elsewhere? -- default if unanswered: No, define inline in version.rs (per OQ-PLN-003)
- Q: Should tests include unit test of struct serialization or only integration test? -- default if unanswered: Integration test only (per OQ-PLN-004)
- Q: Should CLAUDE.md include example JSON output? -- default if unanswered: Yes, one-liner example (per OQ-PLN-005)

## Shared Assumptions

- `serde` and `serde_json` crates are already in dependencies (confirmed)
- Pretty-printed JSON is acceptable (per ASM-005 in requirements.md)
- The literal string "demoswarm" is used for `name` field, not `CARGO_PKG_NAME` (per open_questions.md assumption)
- Python fallback (`runs_tools.py`) update is out of scope for design options but tracked in impact_map.json

---

## Machine Summary
status: VERIFIED

recommended_action: PROCEED
route_to_agent: null
route_to_flow: null

missing_required: []

blockers: []

options_proposed: 3
suggested_default: OPT-001
confidence: High
