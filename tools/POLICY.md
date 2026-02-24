# Tools Policy (YAI)

Non-negotiable:
1) No logic in shell wrappers (wrappers are glue only).
2) Governance tooling in `yai` delegates to canonical `yai-infra` tooling.
3) Tools must never open/merge PRs automatically.
   - Maintainer does PR creation + merge.
   - Agents may create branches + commit + push.

Stability:
- `tools/VERSION` is the tooling interface version (not the repo VERSION).
