# Repo Tooling (YAI)

## What this is
`tools/` is the official interface for repo automation:
- stable entrypoints in `tools/bin/*`
- real logic in `tools/python/yai_tools`

## Why it exists
So workflow stays consistent even when multiple agents touch the repo:
- branch names don’t drift
- PR bodies don’t drift
- exceptions are explicit (`N/A` requires reason)

## Canonical commands
- `tools/bin/yai-dev-issue`
- `tools/bin/yai-dev-branch`
- `tools/bin/yai-dev-pr-body`
- `tools/bin/yai-dev-pr-check`

## Quick usage
Generate issue draft body:

```bash
tools/bin/yai-dev-issue --type runbook --title "Root hardening phase 0.1.0" --mp-id MP-ROOT-HARDENING-0.1.0 --runbook docs/runbooks/root-hardening.md --phase 0.1.0 --out .pr/ISSUE_BODY.md
```

Generate a branch name:

```bash
tools/bin/yai-dev-branch --type feat --issue 123 --area root --desc hardening-forward
```

Generate PR body to a file:

```bash
tools/bin/yai-dev-pr-body --template default --issue 123 --mp-id MP-ROOT-HARDENING-0.1.0 --runbook docs/runbooks/root-hardening.md#phase-0-1-0-protocol-guardrails --classification FEATURE --compatibility A --objective "Enforce protocol guardrails in root runtime" --evidence-positive "happy path handshake succeeds" --evidence-negative "invalid envelope rejects with deterministic error" --command "cargo test -p root_runtime" --out .pr/PR_BODY.md
```

Validate PR body locally:

```bash
tools/bin/yai-dev-pr-check .pr/PR_BODY.md
```

## Maintainer flow (recommended)
- Agents: branch + commits + push
- Maintainer (you): open PR + review + merge

## Traceability gates (ADR ↔ Runbook ↔ MP)

Local:
- Full scan (strict):
  - `make docs-verify`
- PR-like scan (only changed docs):
  - `tools/bin/yai-docs-trace-check --changed --base <BASE_SHA> --head <HEAD_SHA>`

CI:
- Workflow `validate-traceability.yml` runs on PRs that touch ADR/Runbook/MP/docs templates/tools.
- Gate is scoped to changed docs to avoid breaking legacy documents.

Hard rules (when files are touched):
- ADR must have `law_refs` pointing to `deps/yai-specs/...`
- Runbook must have `adr_refs` unless `ops_only=true`
- MP must include: `runbook`, `phase`, `adrs`, `spec_anchors`, `issues`
- MP requires bidirectional link: runbook file must contain the MP id

## Changelog gate (incremental, PR + tag)

Local:
- PR-like validation (default local check):
  - `make changelog-verify`
- Direct invocation:
  - `tools/bin/yai-changelog-check --pr --base <BASE_SHA> --head <HEAD_SHA>`
- Tag simulation:
  - `tools/bin/yai-changelog-check --tag --version "$(cat VERSION)"`

CI:
- Workflow `validate-changelog.yml` runs on PRs and enforces incremental changelog quality.
- Release workflow `bundle.yml` re-validates changelog in tag mode.

Hard rules:
- Non meta/docs-only PRs must update `CHANGELOG.md`.
- Allowed Keep a Changelog subsections only: Added, Changed, Deprecated, Removed, Fixed, Security.
- New/modified changelog lines cannot include placeholders (`TODO`, `TBD`, `...`, `<...>`, `lorem ipsum`).
- Tag `vX.Y.Z` requires `VERSION == X.Y.Z` and a valid `## [X.Y.Z] - YYYY-MM-DD` section with real bullets.
