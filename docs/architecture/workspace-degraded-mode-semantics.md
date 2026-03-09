# Workspace Degraded Mode Semantics (WS-10)

Degraded mode is an explicit runtime state, not an implicit failure.

## Why It Exists

Workspace may request stronger containment than currently enforceable backend capabilities.

## Semantics

- `execution_mode_requested`: requested by workspace envelope.
- `execution_mode_effective`: mode actually enforced by runtime.
- `execution_mode_degraded`: `true` if requested != effective.
- `execution_degraded_reason`: machine-readable reason.
- `execution_unsupported_scopes`: scopes not enforced in current backend.

## Current Reasons

- `containment_not_ready_or_namespace_invalid`
- `isolated_scopes_not_enforced`
- `sandbox_backend_unavailable`
- `none`

## Operator Guidance

- Degraded mode does not mean workspace is unusable.
- It means enforcement level is below requested profile.
- Use inspect/status/debug surfaces to verify effective mode before critical runs.
