# Traceability Map

This is the navigable index that prevents docs drift.
It maps proposal and decision artifacts to contract anchors and delivery evidence.

## How to use this map
- Add a row when a new Proposal, ADR, or Runbook is introduced.
- Keep links as repo-relative paths.
- Anchor every row to real `../governance` paths.

## Proposal -> ADR -> Runbook -> MP map

| Proposal (L2) | ADR targets (L3) | Runbook (L4) | Milestone Packs (L5) |
|---|---|---|---|
| `docs/program/rfc/rfc-runtime-001-runtime-topology-and-authority.md` | `docs/program/adr/adr-runtime-001-single-runtime.md`<br/>`docs/program/adr/adr-runtime-002-root-entrypoint.md`<br/>`docs/program/adr/adr-runtime-003-kernel-authority.md`<br/>`docs/program/adr/adr-orchestration-004-engine-execution.md`<br/>`docs/program/adr/adr-runtime-005-mind-proposer.md` | `docs/archive/legacy/program/milestone-packs/runtime-baselines/operations-foundation/mp-runtime-000-root-hardening.md` | `docs/archive/legacy/program/milestone-packs/root-hardening/mp-runtime-000-root-hardening-v0-1-5.md` |
| `docs/program/rfc/rfc-protocol-002-unified-rpc-and-cli-contract.md` | `docs/program/adr/adr-protocol-006-unified-rpc.md`<br/>`docs/program/adr/adr-contracts-011-contract-runbook-lock.md` | `docs/archive/legacy/program/milestone-packs/runtime-baselines/operations-foundation/mp-runtime-000-contract-runbook-lock.md`<br/>`docs/archive/legacy/program/milestone-packs/runtime-baselines/operations-foundation/mp-runtime-000-root-hardening.md` | `docs/archive/legacy/program/milestone-packs/contract-baseline-lock/mp-contracts-000-contract-runbook-lock-v0-1-4.md` *(planned)*<br/>`docs/archive/legacy/program/milestone-packs/root-hardening/mp-runtime-000-root-hardening-v0-1-5.md` |
| `docs/program/rfc/rfc-workspace-003-workspace-lifecycle-and-isolation.md` | `docs/program/adr/adr-workspace-007-workspace-isolation.md`<br/>`docs/program/adr/adr-workspace-008-connection-lifecycle.md`<br/>`docs/program/adr/adr-orchestration-009-engine-attachment.md`<br/>`docs/program/adr/adr-runtime-010-boot-entrypoint.md` | `docs/archive/legacy/program/milestone-packs/runtime-baselines/workspace/mp-runtime-000-workspaces-lifecycle.md`<br/>`docs/archive/legacy/program/milestone-packs/runtime-baselines/operations-foundation/mp-runtime-000-engine-attach.md` | `docs/archive/legacy/program/milestone-packs/workspaces-lifecycle/mp-workspace-000-workspaces-lifecycle-v0-1-4.md` *(planned)*<br/>`docs/archive/legacy/program/milestone-packs/engine-attach/mp-orchestration-000-engine-attach-v0-1-5.md` *(planned)* |
| `docs/program/rfc/rfc-contracts-004-contract-runbook-lock-and-pin-policy.md` | `docs/program/adr/adr-contracts-011-contract-runbook-lock.md` | `docs/archive/legacy/program/milestone-packs/runtime-baselines/operations-foundation/mp-runtime-000-contract-runbook-lock.md` | `docs/archive/legacy/program/milestone-packs/contract-baseline-lock/mp-contracts-000-contract-runbook-lock-v0-1-4.md` *(planned)* |
| `docs/program/rfc/rfc-contracts-004-contract-runbook-lock-and-pin-policy.md` | `docs/program/adr/adr-contracts-011-contract-runbook-lock.md` | `docs/archive/legacy/program/milestone-packs/runtime-baselines/operations-foundation/mp-runtime-000-specs-refactor-foundation.md` | `docs/archive/legacy/program/milestone-packs/specs-refactor-foundation/mp-governance-000-specs-refactor-foundation-v0-1-8.md` *(planned)* |
| `docs/program/rfc/rfc-formal-005-formal-coverage-roadmap.md` | `docs/program/adr/adr-protocol-006-unified-rpc.md`<br/>`docs/program/adr/adr-contracts-011-contract-runbook-lock.md` | `docs/archive/legacy/program/milestone-packs/runtime-baselines/operations-foundation/mp-runtime-000-contract-runbook-lock.md`<br/>`docs/archive/legacy/program/milestone-packs/runtime-baselines/operations-foundation/mp-runtime-000-root-hardening.md` | `docs/archive/legacy/program/milestone-packs/contract-baseline-lock/mp-contracts-000-contract-runbook-lock-v0-1-4.md` *(planned)*<br/>`docs/archive/legacy/program/milestone-packs/root-hardening/mp-runtime-000-root-hardening-v0-1-5.md` |

## Delivery evidence pointers (L6)

| Track | Tests / Gates / Evidence |
|---|---|
| Contract baseline lock | `ops/evidence/qualification/test-plans/hardfail.md`, runbook commands in `docs/archive/legacy/program/milestone-packs/runtime-baselines/operations-foundation/mp-runtime-000-contract-runbook-lock.md`, CI logs/artifacts |
| Specs refactor foundation | `ops/evidence/qualification/test-plans/README.md`, runbook commands in `docs/archive/legacy/program/milestone-packs/runtime-baselines/operations-foundation/mp-runtime-000-specs-refactor-foundation.md`, CI logs/artifacts |
| Root hardening | `ops/evidence/qualification/test-plans/hardfail.md`, runbook commands in `docs/archive/legacy/program/milestone-packs/runtime-baselines/operations-foundation/mp-runtime-000-root-hardening.md`, CI logs/artifacts |
| Workspaces lifecycle | `ops/evidence/qualification/test-plans/hardfail.md`, runbook commands in `docs/archive/legacy/program/milestone-packs/runtime-baselines/workspace/mp-runtime-000-workspaces-lifecycle.md`, CI logs/artifacts |
| Engine attach | `ops/evidence/qualification/test-plans/hardfail.md`, runbook commands in `docs/archive/legacy/program/milestone-packs/runtime-baselines/operations-foundation/mp-runtime-000-engine-attach.md`, CI logs/artifacts |
| Data plane | `ops/evidence/qualification/test-plans/hardfail.md`, runbook commands in `docs/archive/legacy/program/milestone-packs/runtime-baselines/data-runtime/mp-runtime-000-data-plane.md`, CI logs/artifacts |
| Kernel sovereignty | `ops/evidence/qualification/test-plans/hardfail.md`, runbook commands in `docs/archive/legacy/program/milestone-packs/runtime-baselines/operations-foundation/mp-runtime-000-kernel-sovereignty.md`, CI logs/artifacts |
| Mind Redis STM | `ops/evidence/qualification/test-plans/hardfail.md`, runbook commands in `docs/archive/legacy/program/milestone-packs/runtime-baselines/operations-foundation/mp-runtime-000-mind-redis-stm.md`, CI logs/artifacts |

Notes:
- Keep this map synchronized whenever proposal scope or ADR targets change.
- Do not invent new anchors: always anchor to `../governance` paths.
