# Workspace Containment Structure Model (WS-8/14)

The workspace containment substrate is materialized under `~/.yai/run/<workspace_id>` with first-class, workspace-owned surfaces.

## Canonical workspace-owned structure
- `manifest.json`: identity + lifecycle + root model + containment refs.
- `metadata/binding.json`: workspace-local binding validity view.
- `state/workspace-state.json`: declared/inferred/effective/inspect refs.
- `traces/index.json`: workspace-owned trace index.
- `artifacts/index.json`: workspace-owned artifact index.
- `runtime/runtime-state.json`: workspace-routed runtime local state.

## Ownership and boundaries
- All files above are strictly workspace-scoped.
- Runtime uses global service entrypoints but routes to workspace namespace (`ws/<id>`).
- Cross-workspace access is rejected when active binding scope conflicts.

## Empty-state semantics
- Missing containment surfaces invalidate namespace trust (`containment_surface_missing`).
- Inspect/status expose containment readiness explicitly.
