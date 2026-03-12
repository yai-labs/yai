---
id: ARCH-TRACEABILITY
status: active
effective_date: 2026-02-19
revision: 1
owner: architecture
law_refs:
  - ../governance/foundation/invariants/I-001-traceability.md
---

# Architecture Traceability

<!-- GENERATED FILE: do not edit manually. Run ../infra/tools/bin/yai-architecture-check --write -->

## Component Alignment Map

| Component | Status | ADR | Runbook | MP | L0 anchors |
|---|---|---|---|---|---|
| Boot | implemented | `docs/program/adr/adr-runtime-010-boot-entrypoint.md` | `docs/archive/legacy/program/milestone-packs/runtime-baselines/operations-foundation/mp-runtime-000-root-hardening.md` | `docs/archive/legacy/program/milestone-packs/root-hardening/mp-runtime-000-root-hardening-v0-1-5.md` | `../governance/foundation/invariants/I-001-traceability.md` |
| Engine | implemented | `docs/program/adr/adr-orchestration-004-engine-execution.md`, `docs/program/adr/adr-orchestration-009-engine-attachment.md` | `docs/archive/legacy/program/milestone-packs/runtime-baselines/operations-foundation/mp-runtime-000-engine-attach.md`, `docs/archive/legacy/program/milestone-packs/runtime-baselines/operations-foundation/mp-runtime-000-root-hardening.md` | `docs/archive/legacy/program/milestone-packs/root-hardening/mp-runtime-000-root-hardening-v0-1-5.md` | `../governance/foundation/boundaries/L2-engine.md`, `../governance/contracts/protocol/include/protocol.h` |
| Kernel | implemented | `docs/program/adr/adr-runtime-003-kernel-authority.md`, `docs/program/adr/adr-workspace-007-workspace-isolation.md`, `docs/program/adr/adr-workspace-008-connection-lifecycle.md` | `docs/archive/legacy/program/milestone-packs/runtime-baselines/operations-foundation/mp-runtime-000-root-hardening.md`, `docs/archive/legacy/program/milestone-packs/runtime-baselines/workspace/mp-runtime-000-workspaces-lifecycle.md` | `docs/archive/legacy/program/milestone-packs/root-hardening/mp-runtime-000-root-hardening-v0-1-5.md`, `docs/archive/legacy/program/milestone-packs/root-hardening/mp-runtime-000-root-hardening-v0-1-5.md` | `../governance/foundation/boundaries/L1-kernel.md`, `../governance/contracts/protocol/include/session.h` |
| Mind | partial | `docs/program/adr/adr-runtime-003-kernel-authority.md`, `docs/program/adr/adr-runtime-005-mind-proposer.md` | `docs/archive/legacy/program/milestone-packs/runtime-baselines/operations-foundation/mp-runtime-000-mind-redis-stm.md`, `docs/archive/legacy/program/milestone-packs/runtime-baselines/operations-foundation/mp-runtime-000-root-hardening.md` | `docs/archive/legacy/program/milestone-packs/root-hardening/mp-runtime-000-root-hardening-v0-1-5.md` | `../governance/foundation/boundaries/L3-mind.md`, `../governance/foundation/invariants/I-004-cognitive-reconfiguration.md` |
| Mind-boundaries | partial | `docs/program/adr/adr-runtime-003-kernel-authority.md`, `docs/program/adr/adr-runtime-005-mind-proposer.md` | `docs/archive/legacy/program/milestone-packs/runtime-baselines/operations-foundation/mp-runtime-000-mind-redis-stm.md`, `docs/archive/legacy/program/milestone-packs/runtime-baselines/operations-foundation/mp-runtime-000-root-hardening.md` | `docs/archive/legacy/program/milestone-packs/root-hardening/mp-runtime-000-root-hardening-v0-1-5.md` | `../governance/foundation/boundaries/L3-mind.md`, `../governance/foundation/invariants/I-004-cognitive-reconfiguration.md` |
| Mind-overview | partial | `docs/program/adr/adr-runtime-003-kernel-authority.md`, `docs/program/adr/adr-runtime-005-mind-proposer.md` | `docs/archive/legacy/program/milestone-packs/runtime-baselines/operations-foundation/mp-runtime-000-mind-redis-stm.md`, `docs/archive/legacy/program/milestone-packs/runtime-baselines/operations-foundation/mp-runtime-000-root-hardening.md` | `docs/archive/legacy/program/milestone-packs/root-hardening/mp-runtime-000-root-hardening-v0-1-5.md` | `../governance/foundation/boundaries/L3-mind.md`, `../governance/foundation/invariants/I-004-cognitive-reconfiguration.md` |
| Root | implemented | `docs/program/adr/adr-runtime-001-single-runtime.md`, `docs/program/adr/adr-runtime-002-root-entrypoint.md` | `docs/archive/legacy/program/milestone-packs/runtime-baselines/operations-foundation/mp-runtime-000-root-hardening.md` | `docs/archive/legacy/program/milestone-packs/root-hardening/mp-runtime-000-root-hardening-v0-1-5.md`, `docs/archive/legacy/program/milestone-packs/root-hardening/mp-runtime-000-root-hardening-v0-1-5.md` | `../governance/contracts/protocol/include/auth.h`, `../governance/contracts/protocol/include/transport.h` |
| Vault | partial | `docs/program/adr/adr-runtime-001-single-runtime.md` | `docs/archive/legacy/program/milestone-packs/runtime-baselines/operations-foundation/mp-runtime-000-root-hardening.md` | `docs/archive/legacy/program/milestone-packs/root-hardening/mp-runtime-000-root-hardening-v0-1-5.md` | `../governance/foundation/boundaries/L0-vault.md`, `../governance/contracts/vault/include/yai_vault_abi.h` |

## Notes

- `planned/external` means documented in architecture + ADR but not currently implemented as tracked source in this repository.
- This file is generated from `docs/10-platform/architecture/components/*.md` traceability sections.
