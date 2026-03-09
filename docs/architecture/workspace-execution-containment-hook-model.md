# Workspace Execution Containment Hook Model (WS-10)

This model defines where workspace containment is consumed by execution flow.

## Hook Points

- `workspace.create`
  - accepts optional `--containment-level|--security-level`.
  - persists requested execution mode in workspace manifest.
- `workspace.set|workspace.switch`
  - materializes attach descriptor and execution profile refs.
- `workspace.run`
  - consumes workspace execution profile and injects it into control payload.
  - emits requested/effective/degraded execution mode in runtime reply.
- `workspace.status|workspace.inspect|workspace.policy_effective|workspace.debug_resolution`
  - expose execution requested/effective/degraded state.

## Runtime Contract

- Workspace metadata is no longer passive.
- Runtime reads the envelope and computes:
  - `execution_mode_requested`
  - `execution_mode_effective`
  - `execution_mode_degraded`
  - `execution_degraded_reason`
  - `execution_unsupported_scopes`

## Advisory vs Enforced

- `logical` and `scoped` are currently supported runtime modes.
- `isolated` and `sandboxed` are represented as requested modes but degrade when hard backends are unavailable.
- Degraded semantics are explicit in surfaces and runtime payloads.
