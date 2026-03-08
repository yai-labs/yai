---
id: ARCH-TRACEABILITY
status: active
effective_date: 2026-02-19
revision: 1
owner: architecture
law_refs:
  - deps/law/foundation/invariants/I-001-traceability.md
---

# Architecture Traceability

<!-- GENERATED FILE: do not edit manually. Run ../infra/tools/bin/yai-architecture-check --write -->

## Component Alignment Map

| Component | Status | ADR | Runbook | MP | L0 anchors |
|---|---|---|---|---|---|
| Boot | implemented | `docs/program/22-adr/ADR-010-boot-entrypoint.md` | `docs/program/23-runbooks/root-hardening.md` | `docs/program/24-milestone-packs/root-hardening/MP-ROOT-HARDENING-0.1.0.md` | `deps/law/foundation/invariants/I-001-traceability.md` |
| Engine | implemented | `docs/program/22-adr/ADR-004-engine-execution.md`, `docs/program/22-adr/ADR-009-engine-attachment.md` | `docs/program/23-runbooks/engine-attach.md`, `docs/program/23-runbooks/root-hardening.md` | `docs/program/24-milestone-packs/root-hardening/MP-ROOT-HARDENING-0.1.5.md` | `deps/law/foundation/boundaries/L2-engine.md`, `deps/law/contracts/protocol/include/protocol.h` |
| Kernel | implemented | `docs/program/22-adr/ADR-003-kernel-authority.md`, `docs/program/22-adr/ADR-007-workspace-isolation.md`, `docs/program/22-adr/ADR-008-connection-lifecycle.md` | `docs/program/23-runbooks/root-hardening.md`, `docs/program/23-runbooks/workspaces-lifecycle.md` | `docs/program/24-milestone-packs/root-hardening/MP-ROOT-HARDENING-0.1.2.md`, `docs/program/24-milestone-packs/root-hardening/MP-ROOT-HARDENING-0.1.4.md` | `deps/law/foundation/boundaries/L1-kernel.md`, `deps/law/contracts/protocol/include/session.h` |
| Mind | partial | `docs/program/22-adr/ADR-003-kernel-authority.md`, `docs/program/22-adr/ADR-005-mind-proposer.md` | `docs/program/23-runbooks/mind-redis-stm.md`, `docs/program/23-runbooks/root-hardening.md` | `docs/program/24-milestone-packs/root-hardening/MP-ROOT-HARDENING-0.1.5.md` | `deps/law/foundation/boundaries/L3-mind.md`, `deps/law/foundation/invariants/I-004-cognitive-reconfiguration.md` |
| Mind-boundaries | partial | `docs/program/22-adr/ADR-003-kernel-authority.md`, `docs/program/22-adr/ADR-005-mind-proposer.md` | `docs/program/23-runbooks/mind-redis-stm.md`, `docs/program/23-runbooks/root-hardening.md` | `docs/program/24-milestone-packs/root-hardening/MP-ROOT-HARDENING-0.1.5.md` | `deps/law/foundation/boundaries/L3-mind.md`, `deps/law/foundation/invariants/I-004-cognitive-reconfiguration.md` |
| Mind-overview | partial | `docs/program/22-adr/ADR-003-kernel-authority.md`, `docs/program/22-adr/ADR-005-mind-proposer.md` | `docs/program/23-runbooks/mind-redis-stm.md`, `docs/program/23-runbooks/root-hardening.md` | `docs/program/24-milestone-packs/root-hardening/MP-ROOT-HARDENING-0.1.5.md` | `deps/law/foundation/boundaries/L3-mind.md`, `deps/law/foundation/invariants/I-004-cognitive-reconfiguration.md` |
| Root | implemented | `docs/program/22-adr/ADR-001-single-runtime.md`, `docs/program/22-adr/ADR-002-root-entrypoint.md` | `docs/program/23-runbooks/root-hardening.md` | `docs/program/24-milestone-packs/root-hardening/MP-ROOT-HARDENING-0.1.0.md`, `docs/program/24-milestone-packs/root-hardening/MP-ROOT-HARDENING-0.1.1.md` | `deps/law/contracts/protocol/include/auth.h`, `deps/law/contracts/protocol/include/transport.h` |
| Vault | partial | `docs/program/22-adr/ADR-001-single-runtime.md` | `docs/program/23-runbooks/root-hardening.md` | `docs/program/24-milestone-packs/root-hardening/MP-ROOT-HARDENING-0.1.0.md` | `deps/law/foundation/boundaries/L0-vault.md`, `deps/law/contracts/vault/include/yai_vault_abi.h` |

## Notes

- `planned/external` means documented in architecture + ADR but not currently implemented as tracked source in this repository.
- This file is generated from `docs/10-platform/architecture/components/*.md` traceability sections.
