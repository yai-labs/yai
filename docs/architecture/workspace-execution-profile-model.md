# Workspace Execution Profile Model (WS-10)

Execution profile is the runtime-facing projection of workspace security envelope.

## Fields

- `execution_mode_requested`
- `execution_mode_effective`
- `execution_mode_degraded`
- `execution_degraded_reason`
- `execution_unsupported_scopes`
- `execution_advisory_scopes`
- `process_intent`
- `channel_mode`
- `artifact_policy_mode`
- `network_intent`
- `resource_intent`
- `privilege_intent`
- `attach_descriptor_ref`
- `execution_profile_ref`

## Requested vs Effective

- Requested mode comes from declared envelope (`security_level_declared`).
- Effective mode is computed by runtime considering:
  - containment readiness,
  - namespace validity,
  - backend availability.

Current behavior:

- requested `logical` -> effective `logical`
- requested `scoped` -> effective `scoped`
- requested `isolated` -> degraded to `scoped` when isolated scopes are not enforced
- requested `sandboxed` -> degraded to `scoped` when sandbox backend is unavailable

## Persistence

The profile is persisted in:

- `manifest.json` (`execution_profile` section)
- `runtime/execution-profile.json`
