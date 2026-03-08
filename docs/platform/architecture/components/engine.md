---
id: ARCH-COMP-ENGINE
status: active
effective_date: 2026-02-19
revision: 1
owner: engine
law_refs:
  - deps/law/foundation/boundaries/L2-engine.md
  - deps/law/foundation/invariants/I-002-determinism.md
---

# Engine Component

## Role

Deterministic execution plane for storage/provider/network/resource/cortex operations under Kernel-approved context.

## Current Implementation Status

implemented

## Interfaces and Entry Points

- `engine/src/main.c`
- `engine/src/bridge/rpc_router.c`
- `engine/src/bridge/transport_client.c`
- `engine/src/gates/*.c`

## Authority and Boundary Rules

- Engine must not become an authority decision surface.
- Engine accepts governed command context (`ws_id`, role/arming semantics from upper planes).
- Execution responses must remain deterministic for equivalent inputs.

## Traceability

- ADR refs: `docs/program/22-adr/ADR-004-engine-execution.md`, `docs/program/22-adr/ADR-009-engine-attachment.md`
- Runbook refs: `docs/program/23-runbooks/engine-attach.md`, `docs/program/23-runbooks/root-hardening.md`
- MP refs: `docs/program/24-milestone-packs/root-hardening/MP-ROOT-HARDENING-0.1.5.md`
- L0 anchors: `deps/law/foundation/boundaries/L2-engine.md`, `deps/law/contracts/protocol/include/protocol.h`

## Known Drift / Gaps

- Engine attach lifecycle still marked partial in delivery evidence.

## Next Alignment Steps

- Align engine-attach runbook phases to explicit MP artifacts and CI evidence.
