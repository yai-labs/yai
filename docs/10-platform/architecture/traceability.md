---
id: ARCH-TRACEABILITY
status: active
effective_date: 2026-02-19
revision: 1
owner: architecture
law_refs:
  - deps/yai-specs/contracts/invariants/I-001-traceability.md
---

# Architecture Traceability

<!-- GENERATED FILE: do not edit manually. Run tools/bin/yai-architecture-check --write -->

## Component Alignment Map

| Component | Status | ADR | Runbook | MP | L0 anchors |
|---|---|---|---|---|---|
| Boot | implemented | `docs/20-governance/design/adr/ADR-010-boot-entrypoint.md` | `docs/20-governance/runbooks/root-hardening.md` | `docs/20-governance/milestone-packs/root-hardening/MP-ROOT-HARDENING-0.1.0.md` | `deps/yai-specs/contracts/invariants/I-001-traceability.md` |
| Engine | implemented | `docs/20-governance/design/adr/ADR-004-engine-execution.md`, `docs/20-governance/design/adr/ADR-009-engine-attachment.md` | `docs/20-governance/runbooks/engine-attach.md`, `docs/20-governance/runbooks/root-hardening.md` | `docs/20-governance/milestone-packs/root-hardening/MP-ROOT-HARDENING-0.1.5.md` | `deps/yai-specs/contracts/boundaries/L2-engine.md`, `deps/yai-specs/specs/protocol/include/protocol.h` |
| Kernel | implemented | `docs/20-governance/design/adr/ADR-003-kernel-authority.md`, `docs/20-governance/design/adr/ADR-007-workspace-isolation.md`, `docs/20-governance/design/adr/ADR-008-connection-lifecycle.md` | `docs/20-governance/runbooks/root-hardening.md`, `docs/20-governance/runbooks/workspaces-lifecycle.md` | `docs/20-governance/milestone-packs/root-hardening/MP-ROOT-HARDENING-0.1.2.md`, `docs/20-governance/milestone-packs/root-hardening/MP-ROOT-HARDENING-0.1.4.md` | `deps/yai-specs/contracts/boundaries/L1-kernel.md`, `deps/yai-specs/specs/protocol/include/session.h` |
| Mind | partial | `docs/20-governance/design/adr/ADR-003-kernel-authority.md`, `docs/20-governance/design/adr/ADR-005-mind-proposer.md` | `docs/20-governance/runbooks/mind-redis-stm.md`, `docs/20-governance/runbooks/root-hardening.md` | `docs/20-governance/milestone-packs/root-hardening/MP-ROOT-HARDENING-0.1.5.md` | `deps/yai-specs/contracts/boundaries/L3-mind.md`, `deps/yai-specs/contracts/invariants/I-004-cognitive-reconfiguration.md` |
| Root | implemented | `docs/20-governance/design/adr/ADR-001-single-runtime.md`, `docs/20-governance/design/adr/ADR-002-root-entrypoint.md` | `docs/20-governance/runbooks/root-hardening.md` | `docs/20-governance/milestone-packs/root-hardening/MP-ROOT-HARDENING-0.1.0.md`, `docs/20-governance/milestone-packs/root-hardening/MP-ROOT-HARDENING-0.1.1.md` | `deps/yai-specs/specs/protocol/include/auth.h`, `deps/yai-specs/specs/protocol/include/transport.h` |
| Vault | partial | `docs/20-governance/design/adr/ADR-001-single-runtime.md` | `docs/20-governance/runbooks/root-hardening.md` | `docs/20-governance/milestone-packs/root-hardening/MP-ROOT-HARDENING-0.1.0.md` | `deps/yai-specs/contracts/boundaries/L0-vault.md`, `deps/yai-specs/specs/vault/include/yai_vault_abi.h` |

## Notes

- `planned/external` means documented in architecture + ADR but not currently implemented as tracked source in this repository.
- This file is generated from `docs/10-platform/architecture/components/*.md` traceability sections.
