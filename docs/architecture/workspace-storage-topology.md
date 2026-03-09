# Workspace Storage Topology

## Roots
- Workspace root (human/project root): `~/.yai/workspaces/<workspace_id>` or explicit root.
- Runtime containment root: `~/.yai/run/<workspace_id>`.

## Runtime containment directories
- `metadata/`
- `state/`
- `traces/`
- `artifacts/`
- `runtime/`

## Path invariants
- `runtime_state_root` must equal `~/.yai/run/<workspace_id>`.
- `metadata_root` must equal `~/.yai/run/<workspace_id>/metadata`.
- Surface files must remain inside workspace containment root.
