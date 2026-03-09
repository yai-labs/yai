# Workspace Security Envelope Model (WS-9/14)

The workspace security envelope is the canonical security profile attached to each workspace containment cell.

## Envelope planes
- Declared envelope: operator/system intent (`security_level_declared`).
- Effective envelope: runtime-realized posture (`security_level_effective`).
- Capability plane: features available in current runtime backend.

## Core fields
- `security_envelope_version`
- `security_level_declared`
- `security_level_effective`
- `security_enforcement_mode`
- `security_backend_mode`
- scope flags: process/filesystem/socket/network/resource/privilege/runtime_route/binding
- capability flags: sandbox_ready/hardened_fs/process_isolation/network_policy

## Truth model
- Governance/policy decisions and security envelope are distinct.
- Policy says what is allowed; envelope says how execution is contained.
