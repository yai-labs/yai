# Workflow — YAI Mind

## Issue → Branch → PR

- Create/assign an issue.
- Create a branch:
  - docs/governance work: `docs-gov/...`
  - features: `feat/...`
  - fixes: `fix/...`
- Open a PR early for large work.

## PR requirements

- Scope is tight and explained.
- CI passes (fmt/clippy/test).
- Docs-only branches do not touch `src/`.

## Repository mapping (high level)

- `yai-specs` — canonical contracts, schemas, compliance packs
- `yai` — runtime enforcement, toolchain, runbooks
- `yai-cli` — user interface surface
- `yai-mind` — cognition + memory + providers within L3 boundary
