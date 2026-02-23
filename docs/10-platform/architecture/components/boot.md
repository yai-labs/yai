---
id: ARCH-COMP-BOOT
status: active
effective_date: 2026-02-19
revision: 1
owner: runtime
law_refs:
  - deps/yai-specs/contracts/invariants/I-001-traceability.md
  - deps/yai-specs/contracts/invariants/I-002-determinism.md
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

- ADR refs: `docs/20-governance/design/adr/ADR-010-boot-entrypoint.md`
- Runbook refs: `docs/20-governance/runbooks/root-hardening.md`
- MP refs: `docs/20-governance/milestone-packs/root-hardening/MP-ROOT-HARDENING-0.1.0.md`
- L0 anchors: `deps/yai-specs/contracts/invariants/I-001-traceability.md`

## Known Drift / Gaps

- Some path assumptions differ between boot-initialized sockets and component docs.

## Next Alignment Steps

- Normalize socket contract docs against effective boot/runtime behavior.
