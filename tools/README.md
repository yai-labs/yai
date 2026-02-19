# YAI Repo Tools

This directory contains **official repo tooling**.

Rules:
- `tools/bin/*` are stable entrypoints (wrappers only).
- Real logic lives in `tools/python/yai_tools`.
- No "random scripts" elsewhere in the repo.

Entry points:
- `tools/bin/yai-pr-body` — generate PR body from repo templates (no PR creation).
- `tools/bin/yai-branch` — generate canonical branch names (optional checkout).
- `tools/bin/yai-docs-schema-check` — validate docs frontmatter against local schemas.
- `tools/bin/yai-docs-graph` — generate/check traceability graph and lock.
- `tools/bin/yai-agent-pack` — generate/check canonical machine-readable agent pack.
- `tools/bin/yai-docs-doctor` — run full docs-governance validation stack.

## PR tooling

```bash
tools/bin/yai-pr-body --template docs-governance --issue 123
```
