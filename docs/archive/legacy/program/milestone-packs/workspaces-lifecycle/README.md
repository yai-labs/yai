---
id: MP-WORKSPACES-LIFECYCLE-INDEX
status: active
runbook: docs/archive/legacy/program/milestone-packs/runtime-baselines/workspace/mp-runtime-000-workspaces-lifecycle.md
phase: index
adrs:
  - docs/program/adr/adr-protocol-006-unified-rpc.md
  - docs/program/adr/adr-workspace-007-workspace-isolation.md
  - docs/program/adr/adr-orchestration-009-engine-attachment.md
  - docs/program/adr/adr-contracts-011-contract-runbook-lock.md
  - docs/program/adr/adr-program-012-audit-convergence-gates.md
spec_anchors:
  - ../governance/contracts/control/schema/control_plane.v1.json
  - ../governance/contracts/protocol/include/auth.h
  - ../governance/contracts/vault/include/yai_vault_abi.h
issues:
  - "N/A: workspace-lifecycle track index"
---

# Workspaces Lifecycle Milestone Packs

This track is the Workspace Lifecycle execution line and binds runbook rev4 phases to claims and mandatory evidence commands.

References:
- Runbook: `docs/archive/legacy/program/milestone-packs/runtime-baselines/workspace/mp-runtime-000-workspaces-lifecycle.md`
- Plan: `docs/program/reports/audit-convergence-report.md`
- Matrix: `docs/program/reports/audit-convergence-report.md`
- Claims: `ops/evidence/validation/audits/claims/infra-grammar.v0.1.json`

Policy:
- Mandatory command outcomes are required for closure.
- `SKIP` on mandatory evidence is treated as `FAIL`.

WS-lifecycle sequence (from first):
- `docs/archive/legacy/program/milestone-packs/workspaces-lifecycle/mp-workspace-000-workspaces-lifecycle-v0-1-4.md`
- `docs/archive/legacy/program/milestone-packs/workspaces-lifecycle/mp-workspace-000-workspaces-lifecycle-v0-1-4.md`
- `docs/archive/legacy/program/milestone-packs/workspaces-lifecycle/mp-workspace-000-workspaces-lifecycle-v0-1-4.md`
- `docs/archive/legacy/program/milestone-packs/workspaces-lifecycle/mp-workspace-000-workspaces-lifecycle-v0-1-4.md`
- `docs/archive/legacy/program/milestone-packs/workspaces-lifecycle/mp-workspace-000-workspaces-lifecycle-v0-1-4.md`

## Status Snapshot
- `0.1.0`: open
- `0.1.1`: open
- `0.1.2`: open
- `0.1.3`: open
- `0.1.4`: open


Retention rule: latest-only (intermediate versions are evicted in C17.5).
