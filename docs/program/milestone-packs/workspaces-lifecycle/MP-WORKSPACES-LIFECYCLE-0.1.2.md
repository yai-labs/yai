---
id: MP-WORKSPACES-LIFECYCLE-0.1.2
status: draft
runbook: docs/program/milestone-packs/runtime-baselines/workspaces-lifecycle.md
phase: "0.1.2 — Engine-attached OPEN semantics"
adrs:
  - docs/program/adr/ADR-009-engine-attachment.md
  - docs/program/adr/ADR-008-connection-lifecycle.md
  - docs/program/adr/ADR-012-audit-convergence-gates.md
spec_anchors:
  - ../law/contracts/protocol/include/yai_protocol_ids.h
  - ../law/contracts/control/schema/exec_reply.v1.json
claims:
  - C-DOMAIN-COVERAGE-PROVIDERS
  - C-CONTEXT-PROPAGATION
evidence_commands_required:
  - tools/bin/yai-verify
  - tools/bin/yai-gate
issues:
  - "N/A: workspace-lifecycle phase sequencing"
---

# MP-WORKSPACES-LIFECYCLE-0.1.2

## Metadata
- Runbook: `docs/program/milestone-packs/runtime-baselines/workspaces-lifecycle.md`
- Phase: `0.1.2 — Engine-attached OPEN semantics`
- Status: `draft`

## Objective
Bind `OPEN` semantics to engine-attach functional readiness.

## Mandatory command outcomes
- `tools/bin/yai-verify` -> `PASS`
- `tools/bin/yai-gate` -> `PASS`

Closure policy: mandatory `SKIP` is treated as `FAIL`.

## Definition of Done
- [ ] Lifecycle open reports governance-ready runtime status.
- [ ] Functional engine probe integrated in open criteria.
- [ ] Degraded mode behavior explicit and audited.
- [ ] Evidence links and logs archived.

## Execution Snapshot
- Status: `PLANNED`
- Evidence bundle: `docs/program/milestone-packs/workspaces-lifecycle/evidence/0.1.2/`
