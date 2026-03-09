# Workspace Binding Persistence Model

## Binding layers
- Session-global pointer: `~/.yai/session/active_workspace.json`.
- Workspace-local binding surface: `metadata/binding.json`.

## Semantics
- `set/switch`: write active binding + refresh workspace-local binding state.
- `unset`: remove active binding pointer and clear workspace-local binding marker.
- `clear`: keep binding, clear runtime/effective inferred surfaces.

## Stale/invalid defenses
- Invalid id, missing workspace, root mismatch, containment surface mismatch produce explicit reasons in status/current.
