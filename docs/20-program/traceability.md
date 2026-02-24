# Traceability Map

This is the navigable index that prevents docs drift.
It maps proposal and decision artifacts to contract anchors and delivery evidence.

## How to use this map
- Add a row when a new Proposal, ADR, or Runbook is introduced.
- Keep links as repo-relative paths.
- Anchor every row to real `deps/yai-specs` paths.

## Proposal -> ADR -> Runbook -> MP map

| Proposal (L2) | ADR targets (L3) | Runbook (L4) | Milestone Packs (L5) |
|---|---|---|---|
| `docs/20-program/21-rfc/RFC-001-runtime-topology-and-authority.md` | `docs/20-program/22-adr/ADR-001-single-runtime.md`<br/>`docs/20-program/22-adr/ADR-002-root-entrypoint.md`<br/>`docs/20-program/22-adr/ADR-003-kernel-authority.md`<br/>`docs/20-program/22-adr/ADR-004-engine-execution.md`<br/>`docs/20-program/22-adr/ADR-005-mind-proposer.md` | `docs/20-program/23-runbooks/root-hardening.md` | `docs/20-program/24-milestone-packs/root-hardening/MP-ROOT-HARDENING-0.1.0.md` |
| `docs/20-program/21-rfc/RFC-002-unified-rpc-and-cli-contract.md` | `docs/20-program/22-adr/ADR-006-unified-rpc.md`<br/>`docs/20-program/22-adr/ADR-011-contract-baseline-lock.md` | `docs/20-program/23-runbooks/contract-baseline-lock.md`<br/>`docs/20-program/23-runbooks/root-hardening.md` | `docs/20-program/24-milestone-packs/contract-baseline-lock/MP-CONTRACT-BASELINE-LOCK-0.1.0.md` *(planned)*<br/>`docs/20-program/24-milestone-packs/root-hardening/MP-ROOT-HARDENING-0.1.1.md` |
| `docs/20-program/21-rfc/RFC-003-workspace-lifecycle-and-isolation.md` | `docs/20-program/22-adr/ADR-007-workspace-isolation.md`<br/>`docs/20-program/22-adr/ADR-008-connection-lifecycle.md`<br/>`docs/20-program/22-adr/ADR-009-engine-attachment.md`<br/>`docs/20-program/22-adr/ADR-010-boot-entrypoint.md` | `docs/20-program/23-runbooks/workspaces-lifecycle.md`<br/>`docs/20-program/23-runbooks/engine-attach.md` | `docs/20-program/24-milestone-packs/workspaces-lifecycle/MP-WORKSPACES-LIFECYCLE-0.1.0.md` *(planned)*<br/>`docs/20-program/24-milestone-packs/engine-attach/MP-ENGINE-ATTACH-0.1.0.md` *(planned)* |
| `docs/20-program/21-rfc/RFC-004-contract-baseline-lock-and-pin-policy.md` | `docs/20-program/22-adr/ADR-011-contract-baseline-lock.md` | `docs/20-program/23-runbooks/contract-baseline-lock.md` | `docs/20-program/24-milestone-packs/contract-baseline-lock/MP-CONTRACT-BASELINE-LOCK-0.1.0.md` *(planned)* |
| `docs/20-program/21-rfc/RFC-004-contract-baseline-lock-and-pin-policy.md` | `docs/20-program/22-adr/ADR-011-contract-baseline-lock.md` | `docs/20-program/23-runbooks/specs-refactor-foundation.md` | `docs/20-program/24-milestone-packs/specs-refactor-foundation/MP-SPECS-REFACTOR-FOUNDATION-0.1.0.md` *(planned)* |
| `docs/20-program/21-rfc/RFC-005-formal-coverage-roadmap.md` | `docs/20-program/22-adr/ADR-006-unified-rpc.md`<br/>`docs/20-program/22-adr/ADR-011-contract-baseline-lock.md` | `docs/20-program/23-runbooks/contract-baseline-lock.md`<br/>`docs/20-program/23-runbooks/root-hardening.md` | `docs/20-program/24-milestone-packs/contract-baseline-lock/MP-CONTRACT-BASELINE-LOCK-0.1.3.md` *(planned)*<br/>`docs/20-program/24-milestone-packs/root-hardening/MP-ROOT-HARDENING-0.1.5.md` |

## Delivery evidence pointers (L6)

| Track | Tests / Gates / Evidence |
|---|---|
| Contract baseline lock | `docs/40-qualification/test-plans/hardfail.md`, runbook commands in `docs/20-program/23-runbooks/contract-baseline-lock.md`, CI logs/artifacts |
| Specs refactor foundation | `docs/40-qualification/test-plans/README.md`, runbook commands in `docs/20-program/23-runbooks/specs-refactor-foundation.md`, CI logs/artifacts |
| Root hardening | `docs/40-qualification/test-plans/hardfail.md`, runbook commands in `docs/20-program/23-runbooks/root-hardening.md`, CI logs/artifacts |
| Workspaces lifecycle | `docs/40-qualification/test-plans/hardfail.md`, runbook commands in `docs/20-program/23-runbooks/workspaces-lifecycle.md`, CI logs/artifacts |
| Engine attach | `docs/40-qualification/test-plans/hardfail.md`, runbook commands in `docs/20-program/23-runbooks/engine-attach.md`, CI logs/artifacts |
| Data plane | `docs/40-qualification/test-plans/hardfail.md`, runbook commands in `docs/20-program/23-runbooks/data-plane.md`, CI logs/artifacts |
| Kernel sovereignty | `docs/40-qualification/test-plans/hardfail.md`, runbook commands in `docs/20-program/23-runbooks/kernel-sovereignty.md`, CI logs/artifacts |
| Mind Redis STM | `docs/40-qualification/test-plans/hardfail.md`, runbook commands in `docs/20-program/23-runbooks/mind-redis-stm.md`, CI logs/artifacts |

Notes:
- Keep this map synchronized whenever proposal scope or ADR targets change.
- Do not invent new anchors: always anchor to `deps/yai-specs` paths.
