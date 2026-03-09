# Workspace Degraded-Mode Abuse Model (WS-11)

Degraded mode must be explicit and non-silent.

## Abuse Pattern

Request stronger containment (`isolated` or `sandboxed`) and assume full enforcement while runtime is not capable of it.

## Runtime Contract

- keep `execution_mode_requested`
- compute `execution_mode_effective`
- set `execution_mode_degraded=true` when needed
- expose deterministic `execution_degraded_reason`
- expose `execution_unsupported_scopes`

## Why This Matters

Without these signals, operators can misread policy outcome as full containment enforcement.

## Current Expected Reasons

- `isolated_scopes_not_enforced`
- `sandbox_backend_unavailable`
- `containment_not_ready_or_namespace_invalid`
