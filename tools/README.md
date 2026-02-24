# YAI Repo Tools

This directory contains stable entrypoints only.

Rules:
- `tools/bin/*` are stable wrappers.
- Canonical governance/process tooling lives in `yai-infra/tools/`.
- `yai` keeps only runtime-adjacent tooling required for local build checks.

Use (runtime wrappers kept in yai):
- `tools/bin/yai-check-pins`
- `tools/bin/yai-bundle`
- `tools/bin/yai-verify`
- `tools/bin/yai-gate`
- `tools/bin/yai-suite`
- `tools/bin/yai-docs-trace-check`
- `tools/bin/yai-proof-check`
- `tools/bin/yai-changelog-check`
