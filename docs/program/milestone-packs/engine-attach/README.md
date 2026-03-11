---
id: MP-ENGINE-ATTACH-INDEX
status: active
runbook: docs/program/milestone-packs/runtime-baselines/engine-attach.md
phase: index
adrs:
  - docs/program/adr/ADR-009-engine-attachment.md
  - docs/program/adr/ADR-008-connection-lifecycle.md
  - docs/program/adr/ADR-012-audit-convergence-gates.md
spec_anchors:
  - ../law/contracts/protocol/include/transport.h
  - ../law/contracts/protocol/include/yai_protocol_ids.h
  - ../law/contracts/control/schema/exec_reply.v1.json
issues:
  - "N/A: engine-attach track index"
---

# Engine Attach Milestone Packs

This track is the Engine Attach execution line and binds runbook phases to claim IDs and mandatory evidence commands.

References:
- Plan: `docs/program/reports/audit-convergence/EXECUTION-PLAN-v0.1.0.md`
- Matrix: `docs/program/reports/audit-convergence/AUDIT-CONVERGENCE-MATRIX-v0.1.0.md`
- Claims: `ops/evidence/validation/audits/claims/infra-grammar.v0.1.json`
- Runbook: `docs/program/milestone-packs/runtime-baselines/engine-attach.md`

Policy:
- Mandatory command outcomes are required for closure.
- `SKIP` on mandatory evidence is treated as `FAIL`.

Phase packs (from first):
- `docs/program/milestone-packs/engine-attach/MP-ENGINE-ATTACH-0.1.0.md`
- `docs/program/milestone-packs/engine-attach/MP-ENGINE-ATTACH-0.1.1.md`
- `docs/program/milestone-packs/engine-attach/MP-ENGINE-ATTACH-0.1.2.md`
- `docs/program/milestone-packs/engine-attach/MP-ENGINE-ATTACH-0.1.3.md`
- `docs/program/milestone-packs/engine-attach/MP-ENGINE-ATTACH-0.1.4.md`
- `docs/program/milestone-packs/engine-attach/MP-ENGINE-ATTACH-0.1.5.md`

## Status Snapshot
- `0.1.0`: open
- `0.1.1`: open
- `0.1.2`: open
- `0.1.3`: open
- `0.1.4`: open
- `0.1.5`: open
