---
id: ARCH-COMP-ENGINE
status: active
effective_date: 2026-02-19
revision: 1
owner: engine
law_refs:
  - ../governance/foundation/boundaries/L2-engine.md
  - ../governance/foundation/invariants/I-002-determinism.md
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

- ADR refs: `docs/program/adr/adr-orchestration-004-engine-execution.md`, `docs/program/adr/adr-orchestration-009-engine-attachment.md`
- Runbook refs: `docs/archive/legacy/program/milestone-packs/runtime-baselines/operations-foundation/mp-runtime-000-engine-attach.md`, `docs/archive/legacy/program/milestone-packs/runtime-baselines/operations-foundation/mp-runtime-000-root-hardening.md`
- MP refs: `docs/archive/legacy/program/milestone-packs/root-hardening/mp-runtime-000-root-hardening-v0-1-5.md`
- L0 anchors: `../governance/foundation/boundaries/L2-engine.md`, `../governance/contracts/protocol/include/protocol.h`

## Known Drift / Gaps

- Engine attach lifecycle still marked partial in delivery evidence.

## Next Alignment Steps

- Align engine-attach runbook phases to explicit MP artifacts and CI evidence.
