# CLAUDE.md

This repository contains an SDLC swarm pack under `.claude/`.

**Operational reality:** This file is attached to every agent thread in Claude Code. Treat it as **repo-level policy + shared contracts** (not a marketing doc). If flow commands, agent prompts, or pack-check drift from what's written here, update the pack so everything agrees.

---

## For Humans

This pack provides:

- **6 flows**: Signal → Plan → Build → Gate → Deploy → Wisdom
- **50+ agents**: narrow specialists (requirements-author, code-critic, test-author, *-cleanup, etc.)
- **7 skills**: test-runner, auto-linter, policy-runner, runs-derive, runs-index, openq-tools, secrets-tools

Start here:

```
/flow-1-signal "your feature idea"
```

Then proceed in order (unless you are intentionally running out-of-order):

`/flow-2-plan` → `/flow-3-build` → `/flow-4-gate` → `/flow-5-deploy` → `/flow-6-wisdom`

---

## Operating Model: Swarm Repo

Recommended: run flows in a dedicated `*-swarm` downstream repo.

```
my-project/           # Human workspace (stays calm)
my-project-swarm/     # Swarm workspace (commits freely)
```

Benefits:
- **Inspectability**: `.runs/` artifacts are committed and reviewable
- **Isolation**: swarm activity doesn't disrupt human development
- **Clean PRs**: open PR from swarm to origin when ready

### `.runs/` is Git Content

`.runs/` is committed by default — **do not gitignore it**.

Size discipline:
- Summaries over raw dumps
- No pasting full issue bodies into artifacts
- Keep artifacts "reviewable diff" sized

### Repo Topology (Invariant)

- **Swarm repo (`*-swarm`) is autonomous**. Flows run here end-to-end.
- **Flow 5 (Deploy) merges a run PR into `*-swarm/main`** (the swarm's mainline).
- This pack does **not** merge into the upstream human repo by default.
  (Upstream export is a customization / separate concern.)

---

## Non-Negotiables

These rules exist to prevent drift and "model invention":

1. **Repo root only**
   All commands run from **repo root**; all paths are **repo-root-relative**. Do not rely on `cd`.

2. **No raw git in flow commands or agent prompts**
   Git operations are owned by `repo-operator`. Orchestrators call `repo-operator` using **task phrasing**.
   If you feel compelled to paste git commands into a flow, that's a pack bug.

3. **Control plane vs audit plane**
   Orchestrators route on returned result blocks (`Gate Result`, `Repo Operator Result`).
   Files like `secrets_status.json` and `git_status.md` are durable audit records, not routing inputs.

4. **Two gates for GitHub operations**
   GitHub operations (`gh-issue-manager`, `gh-reporter`) require BOTH:
   - `safe_to_publish: true` (secrets gate)
   - `proceed_to_github_ops: true` (repo hygiene gate)

5. **`run_id` folders never rename**
   Identity changes happen via `canonical_key` + `aliases[]`, never via renaming directories.

You'll see these repeated in the relevant sections on purpose.

---

## Run Identity + State

### Working Directory + Paths Invariant

- All commands run from **repo root**
- All paths are **repo-root-relative**
- Run artifacts live under: `.runs/<run-id>/`
- Flow artifacts live under: `.runs/<run-id>/<flow>/`
- `<flow>` is one of: `signal`, `plan`, `build`, `gate`, `deploy`, `wisdom`

Example: `.runs/feat-auth/build/impl_changes_summary.md`

Code and tests remain in project-defined locations (customize per repo layout).

### `.runs/index.json`

Global run index:

```json
{
  "version": 1,
  "runs": [
    {
      "run_id": "feat-auth",
      "canonical_key": "gh-456",
      "task_key": "feat-auth",
      "task_title": "Add OAuth2 login",
      "issue_number": 456,
      "pr_number": null,
      "updated_at": "2025-12-11T22:15:00Z",
      "status": "VERIFIED",
      "last_flow": "build"
    }
  ]
}
```

Rules:

- One entry per `run_id` (upsert, not append).
- Preserve existing ordering; upsert updates in-place. New runs append.
- Keep fields minimal — counts live in receipts.
- Only these agents may update `.runs/index.json`:
  - `run-prep`, `signal-run-prep`
  - `<flow>-cleanup`
  - `gh-issue-manager`

### `run_meta.json`

Per-run metadata at `.runs/<run-id>/run_meta.json`:

```json
{
  "run_id": "<run-id>",
  "canonical_key": "<gh-456 | pr-789 | null>",
  "aliases": ["<run-id>", "<gh-456>", "<branch-name>"],
  "task_key": "<ticket-id | branch-slug | null>",
  "task_title": "<short normalized title>",
  "created_at": "<ISO8601>",
  "updated_at": "<ISO8601>",
  "iterations": 1,
  "flows_started": ["signal", "plan"],
  "source": "<branch:name | ticket:id | manual>",
  "issue_number": 456,
  "pr_number": null,
  "supersedes": "<previous-run-id | null>",
  "related_runs": []
}
```

Identity rules:

- `run_id` is immutable. **No renames.**
- When a GitHub issue/PR exists, set `canonical_key` and add aliases. Do not rename folders.

---

## Flow Execution Model

### Two State Machines

Every flow uses two complementary state machines:

1. **TodoWrite** = session navigation (ephemeral)
2. **`flow_plan.md`** = durable on-disk state for reruns/handoffs

**Timing rule:** Create TodoWrite immediately. Write/update `flow_plan.md` only **after** `run-prep` / `signal-run-prep` has created `.runs/<run-id>/<flow>/`.

### The Six Flows

| Flow | Slash Command | Inputs | Key Outputs |
|------|---------------|--------|-------------|
| 1. Signal | `/flow-1-signal` | raw feature request | `requirements.md`, `features/*.feature`, `verification_notes.md`, risks, `signal_receipt.json` |
| 2. Plan | `/flow-2-plan` | Signal outputs (if present) | `adr.md`, `api_contracts.yaml`, `observability_spec.md`, `test_plan.md`, `work_plan.md`, `plan_receipt.json` |
| 3. Build | `/flow-3-build` | Plan outputs (if present) | code/tests, critiques, `build_receipt.json` |
| 4. Gate | `/flow-4-gate` | Build outputs (if present) | `merge_decision.md` (verdict: MERGE, BOUNCE, or ESCALATE), `gate_receipt.json` |
| 5. Deploy (Mainline) | `/flow-5-deploy` | Gate outputs (if present) | `deployment_log.md`, `verification_report.md`, `deployment_decision.md`, `deploy_receipt.json` |
| 6. Wisdom | `/flow-6-wisdom` | all prior outputs | `learnings.md`, `feedback_actions.md`, `wisdom_receipt.json` |

**Note on Flow 5:** "Deploy" merges the run PR into `*-swarm/main` (the swarm repo's mainline), not an upstream human repo. This pack treats "mainline promotion" as the Deploy target. Upstream export is a separate concern.

Out-of-order is allowed: proceed best-effort, document assumptions, and expect UNVERIFIED outcomes when upstream artifacts are missing.

---

## Receipts

Receipt naming:

| Flow | Receipt File |
|------|--------------|
| Signal | `.runs/<run-id>/signal/signal_receipt.json` |
| Plan | `.runs/<run-id>/plan/plan_receipt.json` |
| Build | `.runs/<run-id>/build/build_receipt.json` |
| Gate | `.runs/<run-id>/gate/gate_receipt.json` |
| Deploy | `.runs/<run-id>/deploy/deploy_receipt.json` |
| Wisdom | `.runs/<run-id>/wisdom/wisdom_receipt.json` |

Receipt guarantees:

- `counts` are mechanical (grep/wc/parse), never estimated.
- `quality_gates` are sourced from agent Machine Summaries (no recomputation).
- Reporters summarize from receipts, not from raw artifacts.

---

## Machine Summary Contract

Critic and verification agents include a machine-parseable summary block:

```yaml
## Machine Summary
status: VERIFIED | UNVERIFIED | CANNOT_PROCEED

recommended_action: PROCEED | RERUN | BOUNCE | ESCALATE | FIX_ENV
route_to_agent: <agent-name | null>
route_to_flow: <1|2|3|4|5|6 | null>

blockers: []
missing_required: []
concerns: []

can_further_iteration_help: yes | no   # critics only

severity_summary:                      # critics/verifiers
  critical: 0
  major: 0
  minor: 0
```

Semantics:

- `CANNOT_PROCEED` = mechanical failure only (I/O, permissions, tooling unusable). `missing_required` must be non-empty.
- `UNVERIFIED` = gaps/uncertainty/issues documented. `blockers` should explain what prevents VERIFIED.
- `VERIFIED` = adequate for purpose. `blockers` empty.

Routing:

- Orchestrators route on `recommended_action` + `route_to_*`.
- The summary is the control plane. The artifact body is the audit plane.

---

## Control-Plane Blocks (Canonical)

Flows and agents should use these blocks **verbatim** (copy/paste) to avoid schema drift.

### Gate Result (emitted by `secrets-sanitizer`)

<!-- PACK-CONTRACT: GATE_RESULT_V1 START -->
```yaml
## Gate Result
status: CLEAN | FIXED | BLOCKED_PUBLISH
safe_to_commit: true | false
safe_to_publish: true | false
modified_files: true | false
needs_upstream_fix: true | false
recommended_action: PROCEED | RERUN | BOUNCE | ESCALATE | FIX_ENV
route_to_flow: 1 | 2 | 3 | 4 | 5 | 6 | null
route_to_agent: <agent-name> | null
```
<!-- PACK-CONTRACT: GATE_RESULT_V1 END -->

### Repo Operator Result (emitted by `repo-operator`)

<!-- PACK-CONTRACT: REPO_OPERATOR_RESULT_V1 START -->
```yaml
## Repo Operator Result
operation: checkpoint | build | stage | merge | other
status: COMPLETED | COMPLETED_WITH_ANOMALY | FAILED | CANNOT_PROCEED
proceed_to_github_ops: true | false
commit_sha: <sha>
anomaly_paths: []
```
<!-- PACK-CONTRACT: REPO_OPERATOR_RESULT_V1 END -->

Notes:

- `commit_sha` is always populated (current HEAD on no-op).
- Orchestrators route on these returned blocks, not by rereading files.

---

## Canonical Status + Verdict Domains

Do not conflate these domains:

1. **Flow/Agent Status** (Machine Summary + receipts)
   `VERIFIED | UNVERIFIED | CANNOT_PROCEED`

2. **Repo Operator Status** (Repo Operator Result)
   `COMPLETED | COMPLETED_WITH_ANOMALY | FAILED | CANNOT_PROCEED`

3. **Secrets Sanitizer Status** (Gate Result)
   `CLEAN | FIXED | BLOCKED_PUBLISH`

4. **Gate Merge Verdict** (`merge_decision.md`)
   `MERGE | BOUNCE | ESCALATE`

5. **Deploy Verdict** (`deployment_decision.md`)
   `STABLE | NOT_DEPLOYED | BLOCKED_BY_GATE`

6. **Smoke Signal** (runtime signal inside `verification_report.md`)
   `smoke_signal: STABLE | INVESTIGATE | ROLLBACK`

This separation is intentional: "deploy verdict" is conservative and governance-shaped; "smoke signal" is operational signal.

---

## Publish Surface (Per-Flow)

Publish surface = what secrets-sanitizer scans and what repo-operator checkpoints for that flow.

| Flow | Publish Surface |
|------|-----------------|
| 1 | `.runs/<run-id>/signal/`, `.runs/<run-id>/run_meta.json`, `.runs/index.json` |
| 2 | `.runs/<run-id>/plan/`, `.runs/<run-id>/run_meta.json`, `.runs/index.json` |
| 3 | `.runs/<run-id>/build/`, `.runs/<run-id>/run_meta.json`, `.runs/index.json`, plus staged code/test changes |
| 4 | `.runs/<run-id>/gate/`, `.runs/<run-id>/run_meta.json`, `.runs/index.json` |
| 5 | `.runs/<run-id>/deploy/`, `.runs/<run-id>/run_meta.json`, `.runs/index.json` |
| 6 | `.runs/<run-id>/wisdom/`, `.runs/<run-id>/run_meta.json`, `.runs/index.json` |

Key invariant: secrets-sanitizer scans only the current flow's publish surface, not the entire `.runs/<run-id>/`.

---

## Two-Gate Prerequisites for GitHub Operations

GitHub operations (`gh-issue-manager`, `gh-reporter`) require BOTH gates:

| Gate | Source | Field |
|------|--------|-------|
| Secrets Gate | secrets-sanitizer Gate Result | `safe_to_publish: true` |
| Repo Hygiene Gate | repo-operator Result | `proceed_to_github_ops: true` |

If either gate fails:

- skip GH ops
- complete the flow locally
- record why in artifacts
- expect the receipt status to be UNVERIFIED

---

## Git Operations Policy (Repo-Operator Owned)

**Rule (repeat):** do not embed raw git commands in flow commands or agent prompts. All git is executed via `repo-operator` using **task phrasing**.

### Commit Cadence

- **Every flow checkpoints**: audit commit of the flow's publish surface on the run branch.
- **Flow 3 additionally commits code/tests**: the "work product" commit.
- **Flow 5 additionally merges the PR into swarm mainline**: promotion, plus tags/releases if configured.

### Required Tasks (Conceptual)

Exact phrasing is standardized in flow docs:

- ensure run branch: `task: "ensure run branch run/<run-id>"`
- checkpoint allowlist: `task: "checkpoint allowlist for flow <flow>"`
- stage intended changes (Build): `task: "stage intended changes for build"`
- commit/push build changes (Build): `task: "commit and push build changes"`
- merge/tag/release (Deploy release ops): `task: "merge and tag release"`

Safe-bail:

- `checkpoint_mode: local_only` is a repo-operator mode that mechanically forces `proceed_to_github_ops: false` and never pushes.

Anomaly handling:

- If unexpected paths exist outside allowlist (or Build's cleanliness interlock fails), repo-operator:
  - commits only the allowlist when safe (`safe_to_commit: true`)
  - sets `proceed_to_github_ops: false`
  - writes `git_status.md` in the current flow directory

---

## Secrets Sanitizer (Publish Gate)

Execution order in every flow (conceptual):

1. `<flow>-cleanup` writes receipt
2. `secrets-sanitizer` scans publish surface; fixes what it can; returns Gate Result
3. `repo-operator` checkpoints (gated on `safe_to_commit`)
4. `gh-issue-manager` + `gh-reporter` only if both gates pass

Reseal:

- If `modified_files: true`, rerun `(cleanup ↔ secrets-sanitizer)` until it's false.
- If reseal does not converge, safe-bail via repo-operator `checkpoint_mode: local_only` and end the flow UNVERIFIED with evidence.

---

## Flow 5 Distinction: Release Ops vs Reporting Ops

Flow 5 has two categories with different gating:

| Category | Operations | Gating |
|----------|------------|--------|
| Release Ops | merge PR, tag/release | Gate merge verdict = MERGE + repo-operator mechanics |
| Reporting Ops | gh-issue-manager, gh-reporter | Two-gate prerequisites (safe_to_publish + proceed_to_github_ops) |

This distinction prevents "can we post?" from affecting "can we merge?".

---

## Skills

| Skill | Purpose |
|-------|---------|
| `test-runner` | Run tests, capture output to run artifacts |
| `auto-linter` | Format + lint code |
| `policy-runner` | Run policy-as-code checks |
| `runs-derive` | Read-only .runs derivations (counts, Machine Summary extraction, receipt reading) |
| `runs-index` | Write .runs/index.json updates (status, last_flow, updated_at) |
| `openq-tools` | Open questions register (QID generation, append entries) |
| `secrets-tools` | Secrets scanning/redaction for publish gates (never prints secret content) |

---

## CLI Tooling Surface

Rust-based CLI tools replace ad-hoc bash pipelines for deterministic operations.

### Install location (repo-local)

```
.demoswarm/bin/pack-check      # mac/linux
.demoswarm/bin/pack-check.exe  # windows
.demoswarm/bin/demoswarm       # demoswarm CLI (runs-derive, runs-index, openq-tools, secrets-tools)
.demoswarm/bin/demoswarm.exe   # windows
```

### Install (repo-local)

```bash
# Install both tools
cargo install --path tools/demoswarm-pack-check --root .demoswarm
cargo install --path tools/demoswarm-runs-tools --root .demoswarm
```

### Resolver shims

Agents **always invoke via shims** — never assume PATH or direct binary access:

```bash
# Pack validation
bash .claude/scripts/pack-check.sh [OPTIONS]

# Demoswarm CLI operations
bash .claude/scripts/demoswarm.sh <command> [OPTIONS]
```

Shims handle resolution in order:
1. `.demoswarm/bin/<tool>` (repo-local install, preferred)
2. `<tool>` on PATH (global install)
3. `cargo run` fallback (dev only, if `tools/` exists)
4. Python fallback (legacy, `demoswarm.sh` only)

### pack-check

Validates pack structural + contract consistency.

```bash
# Human-readable output
bash .claude/scripts/pack-check.sh --no-color

# Machine-readable JSON
bash .claude/scripts/pack-check.sh --format json
```

### demoswarm

Deterministic helpers for `.runs/` operations. **Agents must use the shim:**

```bash
bash .claude/scripts/demoswarm.sh <command> [options]
```

| Command | Purpose |
|---------|---------|
| `version` | Output version info as JSON (`{"name": "demoswarm", "version": "X.Y.Z"}`) |
| `count pattern --file X --regex Y` | Null-safe grep count |
| `count bdd --dir X` | BDD scenario count |
| `ms get --file X --section Y --key Z` | Extract Machine Summary field |
| `yaml get --file X --key Y` | Extract YAML block field |
| `yaml count-items --file X --item-regex Y` | Count items in YAML array |
| `index upsert-status --index X --run-id Y --status Z` | Update `.runs/index.json` |
| `receipt get --file X --key Y` | Read receipt JSON field |
| `receipts count --run-dir X` | Count receipt files |
| `openapi count-paths --file X` | Count OpenAPI paths |
| `line get --file X --prefix Y` | Extract value from line with prefix |
| `inv get --file X --marker Y` | Extract value from inventory marker line |
| `time now` | ISO8601 timestamp |
| `openq next-id --file X --prefix Y` | Generate next open question ID |
| `openq append --file X --prefix Y --question Z ...` | Append open question entry |
| `secrets scan --path X --output Y` | Scan for secrets (returns status) |
| `secrets redact --file X --type Y` | Redact secrets in-place |

See skill docs for complete reference:
- `.claude/skills/runs-derive/SKILL.md` (read-only derivations)
- `.claude/skills/runs-index/SKILL.md` (index writes)
- `.claude/skills/openq-tools/SKILL.md` (open questions)
- `.claude/skills/secrets-tools/SKILL.md` (secrets scanning)

---

## Customization

See `docs/how-to/customize-pack.md` for:

- prerequisites (bash/jq/grep, Windows/WSL2/Git Bash)
- test/lint command adaptation
- source layout changes
- Git provider adaptation
- policy/security scanner customization

---

## Troubleshooting

### "CANNOT_PROCEED"

CANNOT_PROCEED is mechanical failure only. Fix environment/tooling, then rerun.

### "Microloop won't terminate"

Route on the critic control plane:
- `status: CANNOT_PROCEED` → stop (FIX_ENV)
- `recommended_action: BOUNCE` → follow `route_to_flow/route_to_agent`
- `recommended_action: ESCALATE` → stop microloop; record evidence
- `recommended_action: RERUN` → rerun specified agent
- `recommended_action: PROCEED` → proceed even if UNVERIFIED
- If `recommended_action` absent: use `can_further_iteration_help` as tie-breaker (`no` → proceed; `yes` → rerun)

### "No GitHub update happened"

Check the two gates:

- secrets-sanitizer Gate Result: `safe_to_publish`
- repo-operator Result: `proceed_to_github_ops`

If either is false, GH ops must be skipped.

### "Can't find run by issue number"

Alias resolution is via `.runs/index.json` (`issue_number`/`canonical_key`) and `run_meta.json.aliases[]`. Folder names do not change.
