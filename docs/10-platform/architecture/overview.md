---
id: ARCH-OVERVIEW
status: active
effective_date: 2026-02-19
revision: 1
owner: architecture
law_refs:
  - deps/yai-specs/contracts/axioms/A-002-authority.md
  - deps/yai-specs/contracts/invariants/I-003-governance.md
  - deps/yai-specs/contracts/invariants/I-006-external-effect-boundary.md
---

# Architecture Overview

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

- ADRs: `docs/20-program/22-adr/ADR-001-single-runtime.md`, `docs/20-program/22-adr/ADR-002-root-entrypoint.md`, `docs/20-program/22-adr/ADR-003-kernel-authority.md`, `docs/20-program/22-adr/ADR-004-engine-execution.md`, `docs/20-program/22-adr/ADR-005-mind-proposer.md`
- Runbooks: `docs/20-program/23-runbooks/root-hardening.md`, `docs/20-program/23-runbooks/engine-attach.md`, `docs/20-program/23-runbooks/mind-redis-stm.md`
- MPs: `docs/20-program/24-milestone-packs/root-hardening/MP-ROOT-HARDENING-0.1.0.md`
- L0 anchors: `deps/yai-specs/contracts/boundaries/L1-kernel.md`, `deps/yai-specs/contracts/boundaries/L2-engine.md`, `deps/yai-specs/contracts/boundaries/L3-mind.md`
