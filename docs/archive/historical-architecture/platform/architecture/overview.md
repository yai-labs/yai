---
id: ARCH-OVERVIEW
status: historical
effective_date: 2026-02-19
revision: 1
owner: architecture
law_refs:
  - ../governance/foundation/axioms/A-002-authority.md
  - ../governance/foundation/invariants/I-003-governance.md
  - ../governance/foundation/invariants/I-006-external-effect-boundary.md
---

# Architecture Overview

> Historical overview: pre-cutover topology record.
> Authoritative runtime model is `cli -> sdk -> yai` with one ingress socket.

## Role

Define the current machine-runtime architecture of `yai`, separating implemented reality from target trajectory.

## Canonical Topology

Canonical Topology: Root -> Kernel -> Engine with Mind as planned/external L3 plane.

## Current implemented reality

- Root plane is implemented in C (`root/src/yai_root_server.c`) and exposes machine ingress.
- Kernel authority plane is implemented in C (`kernel/src/core/*`, `kernel/src/bin/workspace_kernel_main.c`).
- Engine execution plane is implemented in C (`engine/src/*`) and reaches Root via transport client.
- Boot entrypoint exists (`boot/src/yai_boot_main.c`) and launches Root + Kernel.
- Mind implementation is not present in tracked source under `yai/mind` (current tree is build artifacts only).

## Target architecture (aligned to ADR intent)

- Single machine runtime with governed ingress at Root.
- Authority centralized in Kernel.
- Deterministic effect execution in Engine under Kernel governance.
- Mind remains proposal-only and external/planned until implementation is restored.

## Traceability

- ADRs: `docs/program/adr/adr-runtime-001-single-runtime.md`, `docs/program/adr/adr-runtime-002-root-entrypoint.md`, `docs/program/adr/adr-runtime-003-kernel-authority.md`, `docs/program/adr/adr-orchestration-004-engine-execution.md`, `docs/program/adr/adr-runtime-005-mind-proposer.md`
- Runbooks: `docs/archive/legacy/program/milestone-packs/runtime-baselines/operations-foundation/mp-runtime-000-root-hardening.md`, `docs/archive/legacy/program/milestone-packs/runtime-baselines/operations-foundation/mp-runtime-000-engine-attach.md`, `docs/archive/legacy/program/milestone-packs/runtime-baselines/operations-foundation/mp-runtime-000-mind-redis-stm.md`
- MPs: `docs/archive/legacy/program/milestone-packs/root-hardening/mp-runtime-000-root-hardening-v0-1-5.md`
- L0 anchors: `../governance/foundation/boundaries/L1-kernel.md`, `../governance/foundation/boundaries/L2-engine.md`, `../governance/foundation/boundaries/L3-mind.md`
