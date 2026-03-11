# B7 Migration Marker

Status: complete (implementation cutover)

## Applied

- Canonical contract surfaces absorbed into `governance/contracts/`.
- Canonical structural schemas absorbed into `governance/schema/`.
- Governance compatibility checks now enforce contract/schema presence.
- Tooling and tests switched to governance-first contract/schema paths.

## Residual Compatibility

- External law-root based contract/schema paths remain fallback-only for
  backward compatibility in scripts and local environments.
