---
id: ARCH-COMP-KERNEL
status: active
effective_date: 2026-02-19
revision: 1
owner: kernel
law_refs:
  - deps/yai-specs/contracts/boundaries/L1-kernel.md
  - deps/yai-specs/contracts/invariants/I-006-external-effect-boundary.md
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

- ADR refs: `docs/design/adr/ADR-003-kernel-authority.md`, `docs/design/adr/ADR-007-workspace-isolation.md`, `docs/design/adr/ADR-008-connection-lifecycle.md`
- Runbook refs: `docs/runbooks/root-hardening.md`, `docs/runbooks/workspaces-lifecycle.md`
- MP refs: `docs/milestone-packs/root-hardening/MP-ROOT-HARDENING-0.1.2.md`, `docs/milestone-packs/root-hardening/MP-ROOT-HARDENING-0.1.4.md`
- L0 anchors: `deps/yai-specs/contracts/boundaries/L1-kernel.md`, `deps/yai-specs/specs/protocol/include/session.h`

## Known Drift / Gaps

- Runtime behavior and lifecycle docs are ahead of fully proven non-skip evidence for all phases.

## Next Alignment Steps

- Close remaining workspace/engine integration MPs with non-skip evidence.
