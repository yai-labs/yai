# YAI Repo Tools

This directory contains stable entrypoints only.

Rules:
- `tools/bin/*` are stable wrappers.
- Canonical governance/process tooling lives in `yai-infra/tools/`.
- `yai` keeps only runtime-adjacent tooling required for local build checks.

Use:
- `tools/bin/yai-check-pins`
- `tools/bin/yai-bundle`
- `tools/bin/yai-verify`
- `tools/bin/yai-gate`
- `tools/bin/yai-suite`
