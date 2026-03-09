# Workspace Filesystem Scope Model

Workspace-owned filesystem surfaces are under `~/.yai/run/<workspace_id>` and validated through containment checks.
This provides scoped containment and anti-collision guarantees for state/traces/artifacts/runtime-local data.
