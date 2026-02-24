# Tools Policy (YAI)

Non-negotiable:
1) No non-core governance logic in `yai` wrappers.
2) Non-core tooling delegates to canonical `yai-infra` targets.
3) Tools must never open/merge PRs automatically.

Runtime note:
- Runtime build integrity checks may remain local under `tools/dev/` when strictly required by core build CI.
