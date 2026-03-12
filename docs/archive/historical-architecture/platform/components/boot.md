---
id: ARCH-COMP-BOOT
status: active
effective_date: 2026-02-19
revision: 1
owner: runtime
law_refs:
  - ../governance/foundation/invariants/I-001-traceability.md
  - ../governance/foundation/invariants/I-002-determinism.md
---

# Boot Component

## Role

Machine entrypoint that validates environment and launches baseline runtime planes.

## Current Implementation Status

implemented

## Interfaces and Entry Points

- `boot/src/yai_boot_main.c`
- `boot/src/preboot.c`
- `boot/src/bootstrap.c`

## Authority and Boundary Rules

- Boot initializes runtime surfaces but does not perform policy decisions.
- Boot must not bypass Root/Kernel lifecycle ordering.

## Traceability

- ADR refs: `docs/program/adr/adr-runtime-010-boot-entrypoint.md`
- Runbook refs: `docs/archive/legacy/program/milestone-packs/runtime-baselines/operations-foundation/mp-runtime-000-root-hardening.md`
- MP refs: `docs/archive/legacy/program/milestone-packs/root-hardening/mp-runtime-000-root-hardening-v0-1-5.md`
- L0 anchors: `../governance/foundation/invariants/I-001-traceability.md`

## Known Drift / Gaps

- Some path assumptions differ between boot-initialized sockets and component docs.

## Next Alignment Steps

- Normalize socket contract docs against effective boot/runtime behavior.
