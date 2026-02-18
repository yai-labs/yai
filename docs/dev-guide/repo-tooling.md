# Repo Tooling (YAI)

## What this is
`tools/` is the official interface for repo automation:
- stable entrypoints in `tools/bin/*`
- real logic in `tools/python/yai_tools`

## Why it exists
So workflow stays consistent even when multiple agents touch the repo:
- branch names don’t drift
- PR bodies don’t drift
- exceptions are explicit (N/A requires reason)

## Quick usage
Generate a branch name:

```bash
tools/bin/yai-branch --type feat --issue 123 --area root --desc hardening-forward
```

Create and checkout branch:

```bash
tools/bin/yai-branch --type feat --issue 123 --area root --desc hardening-forward --checkout
```

Generate PR body to a file:

```bash
tools/bin/yai-pr-body --template default --issue 123 --out /tmp/pr.md
```

Meta PR body (allowed only for bootstrap) requires reason:

```bash
tools/bin/yai-pr-body --template docs-governance --issue N/A --reason "meta bootstrap" --out /tmp/pr.md
```

## Maintainer flow (recommended)
- Agents: branch + commits + push
- Maintainer (you): open PR + review + merge
