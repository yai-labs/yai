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
