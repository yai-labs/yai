---
id: ARCH-COMP-KERNEL
status: active
effective_date: 2026-02-19
revision: 1
owner: kernel
law_refs:
  - ../governance/foundation/boundaries/L1-kernel.md
  - ../governance/foundation/invariants/I-006-external-effect-boundary.md
---

# Kernel Component

## Role

Authority plane responsible for policy enforcement, session control, and deterministic command validation.

## Current Implementation Status

implemented

## Interfaces and Entry Points

- `kernel/src/bin/workspace_kernel_main.c`
- `kernel/src/core/rpc_binary.c`
- `kernel/src/core/yai_session.c`
- `kernel/src/enforcement/enforcement.c`

## Authority and Boundary Rules

- Kernel is the authoritative gate for role/arming/session validity.
- Must reject out-of-contract requests deterministically.
- Must not delegate authority checks to Engine.

## Traceability

- ADR refs: `docs/program/adr/adr-runtime-003-kernel-authority.md`, `docs/program/adr/adr-workspace-007-workspace-isolation.md`, `docs/program/adr/adr-workspace-008-connection-lifecycle.md`
- Runbook refs: `docs/archive/legacy/program/milestone-packs/runtime-baselines/operations-foundation/mp-runtime-000-root-hardening.md`, `docs/archive/legacy/program/milestone-packs/runtime-baselines/workspace/mp-runtime-000-workspaces-lifecycle.md`
- MP refs: `docs/archive/legacy/program/milestone-packs/root-hardening/mp-runtime-000-root-hardening-v0-1-5.md`, `docs/archive/legacy/program/milestone-packs/root-hardening/mp-runtime-000-root-hardening-v0-1-5.md`
- L0 anchors: `../governance/foundation/boundaries/L1-kernel.md`, `../governance/contracts/protocol/include/session.h`

## Known Drift / Gaps

- Runtime behavior and lifecycle docs are ahead of fully proven non-skip evidence for all phases.

## Next Alignment Steps

- Close remaining workspace/engine integration MPs with non-skip evidence.
