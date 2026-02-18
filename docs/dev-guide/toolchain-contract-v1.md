# Toolchain Contract v1 (YAI)
Effective: 2026-02-18
Status: Active
Owner: maintainer

## Why
No PR is accepted without traceability and deterministic evidence.

## Branch policy
- Default: branch must be linked to an issue.
- Allowed no-issue exceptions:
  - `meta/*` (governance/tooling/docs-only)
  - `hotfix/*` (must include explicit reason in PR)

## PR metadata minimum (blocking)
Every PR must include:
- `Issue-ID: #<number>` or `N/A` (+ `Issue-Reason`)
- `MP-ID: MP-...` or `N/A`
- `Runbook: docs/runbooks/<name>.md#<anchor>` or `N/A`
- `Base-Commit: <40-char-sha>`
- `Classification`, `Compatibility`
- `Evidence` section (non-empty, no TODO/TBD)
- `Commands run` with fenced `bash` block

## Tooling commands
- `tools/bin/yai-dev-issue`
- `tools/bin/yai-dev-branch`
- `tools/bin/yai-dev-pr-body`
- `tools/bin/yai-dev-pr-check`

## Repository settings checklist (manual)
- Require status checks to pass before merging
- Require branches to be up to date
- Require conversation resolution
- Restrict who can push to `main`
- Optional: Require linear history
- Optional: Auto-delete branches
