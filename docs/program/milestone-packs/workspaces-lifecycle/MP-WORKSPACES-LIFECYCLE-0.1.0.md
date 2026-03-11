---
id: MP-WORKSPACES-LIFECYCLE-0.1.0
status: draft
runbook: docs/program/milestone-packs/runtime-baselines/workspaces-lifecycle.md
phase: "0.1.0 — Workspace layout baseline"
adrs:
  - docs/program/adr/ADR-007-workspace-isolation.md
  - docs/program/adr/ADR-010-boot-entrypoint.md
  - docs/program/adr/ADR-012-audit-convergence-gates.md
spec_anchors:
  - ../law/contracts/control/schema/control_plane.v1.json
  - ../law/contracts/vault/include/yai_vault_abi.h
claims:
  - C-DOMAIN-COVERAGE-RESOURCE
  - C-KERNEL-HARD-BOUNDARY-CORE
evidence_commands_required:
  - tools/bin/yai-check-pins
  - tools/bin/yai-verify
  - tools/bin/yai-gate
issues:
  - "N/A: workspace-lifecycle phase sequencing"
---

# MP-WORKSPACES-LIFECYCLE-0.1.0

## Metadata
- Runbook: `docs/program/milestone-packs/runtime-baselines/workspaces-lifecycle.md`
- Phase: `0.1.0 — Workspace layout baseline`
- Status: `draft`

## Objective
Establish deterministic workspace skeleton and manifest governance contract.

## Mandatory command outcomes
- `tools/bin/yai-check-pins` -> `PASS`
- `tools/bin/yai-verify` -> `PASS`
- `tools/bin/yai-gate` -> `PASS`

Closure policy: mandatory `SKIP` is treated as `FAIL`.

## Definition of Done
- [ ] Layout and manifest are idempotent.
- [ ] Invalid/partial layout is hard rejected.
- [ ] Trace events emitted for lifecycle creation path.
- [ ] MP links from runbook phase and matrix remain valid.

## Execution Snapshot
- Status: `PLANNED`
- Evidence bundle: `docs/program/milestone-packs/workspaces-lifecycle/evidence/0.1.0/`
