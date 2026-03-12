---
id: MP-ENGINE-ATTACH-INDEX
status: active
runbook: docs/archive/legacy/program/milestone-packs/runtime-baselines/operations-foundation/mp-runtime-000-engine-attach.md
phase: index
adrs:
  - docs/program/adr/adr-orchestration-009-engine-attachment.md
  - docs/program/adr/adr-workspace-008-connection-lifecycle.md
  - docs/program/adr/adr-program-012-audit-convergence-gates.md
spec_anchors:
  - ../governance/contracts/protocol/include/transport.h
  - ../governance/contracts/protocol/include/yai_protocol_ids.h
  - ../governance/contracts/control/schema/exec_reply.v1.json
issues:
  - "N/A: engine-attach track index"
---

# Engine Attach Milestone Packs

This track is the Engine Attach execution line and binds runbook phases to claim IDs and mandatory evidence commands.

References:
- Plan: `docs/program/reports/audit-convergence-report.md`
- Matrix: `docs/program/reports/audit-convergence-report.md`
- Claims: `ops/evidence/validation/audits/claims/infra-grammar.v0.1.json`
- Runbook: `docs/archive/legacy/program/milestone-packs/runtime-baselines/operations-foundation/mp-runtime-000-engine-attach.md`

Policy:
- Mandatory command outcomes are required for closure.
- `SKIP` on mandatory evidence is treated as `FAIL`.

Phase packs (from first):
- `docs/archive/legacy/program/milestone-packs/engine-attach/mp-orchestration-000-engine-attach-v0-1-5.md`
- `docs/archive/legacy/program/milestone-packs/engine-attach/mp-orchestration-000-engine-attach-v0-1-5.md`
- `docs/archive/legacy/program/milestone-packs/engine-attach/mp-orchestration-000-engine-attach-v0-1-5.md`
- `docs/archive/legacy/program/milestone-packs/engine-attach/mp-orchestration-000-engine-attach-v0-1-5.md`
- `docs/archive/legacy/program/milestone-packs/engine-attach/mp-orchestration-000-engine-attach-v0-1-5.md`
- `docs/archive/legacy/program/milestone-packs/engine-attach/mp-orchestration-000-engine-attach-v0-1-5.md`

## Status Snapshot
- `0.1.0`: open
- `0.1.1`: open
- `0.1.2`: open
- `0.1.3`: open
- `0.1.4`: open
- `0.1.5`: open


Retention rule: latest-only (intermediate versions are evicted in C17.5).
