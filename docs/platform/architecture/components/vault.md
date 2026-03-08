---
id: ARCH-COMP-VAULT
status: active
effective_date: 2026-02-19
revision: 1
owner: runtime
law_refs:
  - deps/law/foundation/boundaries/L0-vault.md
  - deps/law/contracts/vault/include/yai_vault_abi.h
---

# Vault Component

## Role

L0-adjacent ABI/state boundary used by runtime planes for governed identity/state exchange.

## Current Implementation Status

partial

## Interfaces and Entry Points

- `kernel/include/yai_vault.h`
- `boot/src/bootstrap.c`
- `engine/src/bridge/bridge.c`

## Authority and Boundary Rules

- Vault is not an authority decision plane.
- Vault contract and ABI are normative from `deps/law`.

## Traceability

- ADR refs: `docs/program/22-adr/ADR-001-single-runtime.md`
- Runbook refs: `docs/program/23-runbooks/root-hardening.md`
- MP refs: `docs/program/24-milestone-packs/root-hardening/MP-ROOT-HARDENING-0.1.0.md`
- L0 anchors: `deps/law/foundation/boundaries/L0-vault.md`, `deps/law/contracts/vault/include/yai_vault_abi.h`

## Known Drift / Gaps

- Dedicated vault formalization and cross-plane evidence remain partial.

## Next Alignment Steps

- Tighten vault ABI lifecycle checks in proof/evidence paths.
