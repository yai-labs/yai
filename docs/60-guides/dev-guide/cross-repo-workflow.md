# Cross-Repo Workflow

YAI governance spans multiple repos (`yai`, `yai-specs`, `yai-cli`, `yai-mind`).

Rules:
- Keep normative spec changes in `yai-specs` branches.
- Keep runtime enforcement and program docs in `yai` branches.
- Link related PRs/issues with explicit dependency notes.
- Update pins only after source branches are merged.
