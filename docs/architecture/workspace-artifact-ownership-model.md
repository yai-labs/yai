# Workspace Artifact Ownership Model

Artifacts are workspace-owned and indexed at `artifacts/index.json`.

## Rules
- Artifact refs are stored per-workspace.
- Artifact index does not cross-link other workspace roots.
- Clear-state updates runtime refs without rebinding workspace session context.
