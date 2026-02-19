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

## Component Alignment Map

| Component | Status | ADR | Runbook | MP | L0 anchors |
|---|---|---|---|---|---|
| Root | implemented | `docs/design/adr/ADR-001-single-runtime.md`, `docs/design/adr/ADR-002-root-entrypoint.md` | `docs/runbooks/root-hardening.md` | `docs/milestone-packs/root-hardening/MP-ROOT-HARDENING-0.1.0.md` | `deps/yai-specs/specs/protocol/include/transport.h`, `deps/yai-specs/contracts/invariants/I-003-governance.md` |
| Kernel | implemented | `docs/design/adr/ADR-003-kernel-authority.md`, `docs/design/adr/ADR-007-workspace-isolation.md` | `docs/runbooks/root-hardening.md`, `docs/runbooks/workspaces-lifecycle.md` | `docs/milestone-packs/root-hardening/MP-ROOT-HARDENING-0.1.2.md` | `deps/yai-specs/contracts/boundaries/L1-kernel.md`, `deps/yai-specs/contracts/invariants/I-006-external-effect-boundary.md` |
| Engine | implemented | `docs/design/adr/ADR-004-engine-execution.md`, `docs/design/adr/ADR-009-engine-attachment.md` | `docs/runbooks/engine-attach.md` | *(TBD)* | `deps/yai-specs/contracts/boundaries/L2-engine.md`, `deps/yai-specs/specs/protocol/include/protocol.h` |
| Boot | implemented | `docs/design/adr/ADR-010-boot-entrypoint.md` | `docs/runbooks/root-hardening.md` | `docs/milestone-packs/root-hardening/MP-ROOT-HARDENING-0.1.0.md` | `deps/yai-specs/contracts/invariants/I-001-traceability.md` |
| Vault | partial | `docs/design/adr/ADR-001-single-runtime.md` | `docs/runbooks/root-hardening.md` | `docs/milestone-packs/root-hardening/MP-ROOT-HARDENING-0.1.0.md` | `deps/yai-specs/contracts/boundaries/L0-vault.md`, `deps/yai-specs/specs/vault/include/yai_vault_abi.h` |
| Mind | planned/external | `docs/design/adr/ADR-005-mind-proposer.md` | `docs/runbooks/mind-redis-stm.md` | *(TBD)* | `deps/yai-specs/contracts/boundaries/L3-mind.md`, `deps/yai-specs/contracts/invariants/I-004-cognitive-reconfiguration.md` |

## Notes

- `planned/external` means documented in architecture + ADR but not currently implemented as tracked source in this repository.
- Keep this map updated when component status changes.
