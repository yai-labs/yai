# Workspace Attach Descriptor Model (WS-10)

Attach descriptor is the execution-containment snapshot used during workspace attach/switch.

## Surface

- Path: `runtime/attach-descriptor.json`
- Type: `yai.workspace.attach.descriptor.v1`

## Descriptor Fields

- `workspace_id`
- `binding_scope` (`session`)
- `runtime_attached`
- `control_plane_attached`
- `channel_mode`
- `artifact_policy_mode`
- `process_intent`
- `mode_requested`
- `mode_effective`

## Role

- Bridges lifecycle (`set/switch`) and execution (`run`).
- Makes attach semantics explicit and inspectable.
- Prepares future hard backends without changing high-level command grammar.
