# Secrets Scan Report

## Status: CLEAN

## Scope
- Allowlist scanned: `.runs/cli-version-cmd/build/`, `.runs/cli-version-cmd/run_meta.json`, `.runs/index.json`
- Staged files scanned: 14
- Notes: Cargo.lock is auto-generated (no manual secrets possible); all other files reviewed

## Findings (redacted)
| # | Type | File | Line | Action |
|---|------|------|------|--------|

(No findings)

## Actions Taken
### Redacted
- None

### Externalized
- None

### Unstaged
- None

## Safety Flags
- safe_to_commit: true
- safe_to_publish: true
- needs_upstream_fix: false
- recommended_action: PROCEED
- route_to_flow: null
- route_to_agent: null

## Notes
- All 16 allowlist artifacts scanned (14 build artifacts + run_meta.json + index.json)
- All 14 staged files scanned (Rust source, tests, Cargo.lock, CLAUDE.md)
- Secret patterns checked: GitHub tokens, AWS access keys, Stripe live keys, private keys, JWT tokens
- No secrets detected on publish surface
