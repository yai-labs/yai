# Workspace Prompt Context

## Goal

Provide a compact, stable prompt-facing context payload derived from current workspace binding.

This is not full inspect output. It is a lightweight session summary.

## Contract

Runtime command:

- `command_id: yai.workspace.prompt_context`

Payload result type:

- `yai.workspace.prompt_context.v1`

Primary fields:

- `binding_status` (`active|no_active|stale|invalid`)
- `workspace_id` (when active)
- `workspace_alias` (when active)
- `state` (workspace lifecycle summary)
- `declared.family` (if available)
- `declared.specialization` (if available)
- `reason` (for stale/invalid)

## Design constraints

- small payload
- stable keys
- fast derivation from session + workspace metadata
- no heavy resolution trace data

## Activate/current/clear behavior

- `activate`: writes session binding to selected workspace id.
- `current`: resolves active binding and returns active/no_active/stale/invalid.
- `clear`: removes binding and returns `no_active`.

## Edge cases

- missing binding file -> `no_active`
- malformed binding id -> `invalid`
- binding to missing workspace manifest -> `stale`
- env override `YAI_ACTIVE_WORKSPACE` takes precedence over binding file

## Next step readiness

This prompt contract is the base for WS-3 inspect/status flows and shell prompt adapters.
