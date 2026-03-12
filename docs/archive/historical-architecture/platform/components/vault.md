---
id: ARCH-COMP-VAULT
status: active
effective_date: 2026-02-19
revision: 1
owner: runtime
law_refs:
  - ../governance/foundation/boundaries/L0-vault.md
  - ../governance/contracts/vault/include/yai_vault_abi.h
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
- Vault contract and ABI are normative from `../governance`.

## Traceability

- ADR refs: `docs/program/adr/adr-runtime-001-single-runtime.md`
- Runbook refs: `docs/archive/legacy/program/milestone-packs/runtime-baselines/operations-foundation/mp-runtime-000-root-hardening.md`
- MP refs: `docs/archive/legacy/program/milestone-packs/root-hardening/mp-runtime-000-root-hardening-v0-1-5.md`
- L0 anchors: `../governance/foundation/boundaries/L0-vault.md`, `../governance/contracts/vault/include/yai_vault_abi.h`

## Known Drift / Gaps

- Dedicated vault formalization and cross-plane evidence remain partial.

## Next Alignment Steps

- Tighten vault ABI lifecycle checks in proof/evidence paths.
