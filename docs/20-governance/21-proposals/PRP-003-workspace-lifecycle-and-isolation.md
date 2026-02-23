---
id: PRP-003
title: Workspace lifecycle and isolation guarantees
status: draft
owner: runtime-kernel
effective_date: 2026-02-19
revision: 1
supersedes: []
related:
  adr:
    - docs/20-governance/22-adr/ADR-007-workspace-isolation.md
    - docs/20-governance/22-adr/ADR-008-connection-lifecycle.md
    - docs/20-governance/22-adr/ADR-009-engine-attachment.md
    - docs/20-governance/22-adr/ADR-010-boot-entrypoint.md
  runbooks:
    - docs/20-governance/23-runbooks/workspaces-lifecycle.md
    - docs/20-governance/23-runbooks/engine-attach.md
  milestone_packs: []
  specs:
    - deps/yai-specs/contracts/boundaries/L1-kernel.md
    - deps/yai-specs/specs/protocol/include/session.h
    - deps/yai-specs/contracts/invariants/I-002-determinism.md
    - deps/yai-specs/contracts/invariants/I-006-external-effect-boundary.md
tags:
  - workspace
  - isolation
  - lifecycle
---

# PRP-003 - Workspace lifecycle and isolation guarantees

## Problem
Workspace lifecycle behavior is currently captured at ADR level but lacks a consolidated pre-decision analysis that spans lock, connection lifecycle, boot/attach flow, and runtime routing.

## Scope
- In scope: Workspace lock semantics, connection lifecycle, attach/boot boundaries, routing constraints.
- Out of scope: Protocol envelope fields and formal-coverage expansion.

## Proposed Change
Define one lifecycle proposal that aligns isolation guarantees with Kernel boundary rules and explicitly traces lifecycle states to contract invariants.

## Options Compared
- Option A: Lockfile-first lifecycle with incremental runtime registry transition.
- Option B: Immediate runtime-registry lifecycle with lockfiles deprecated.

## Risks
- Race conditions in transition path. Mitigation: explicit state machine and deterministic reject tests.
- Operator confusion during mixed mode. Mitigation: docs and runbook acceptance checks.

## Rollout Sketch
1. Define lifecycle states and invalid transitions.
2. Map each transition to spec/invariant anchors.
3. Gate rollout with workspace lifecycle runbook phases.

## Exit Criteria
- [ ] Lifecycle state model is documented with valid/invalid transitions.
- [ ] Isolation guarantees are mapped to boundary and invariant anchors.
- [ ] ADR mapping set (007..010) is explicit.

## Traceability

- Spec anchors (if any): `deps/yai-specs/contracts/boundaries/L1-kernel.md`, `deps/yai-specs/specs/protocol/include/session.h`, `deps/yai-specs/contracts/invariants/I-002-determinism.md`, `deps/yai-specs/contracts/invariants/I-006-external-effect-boundary.md`
- Targets ADR: `docs/20-governance/22-adr/ADR-007-workspace-isolation.md`, `docs/20-governance/22-adr/ADR-008-connection-lifecycle.md`, `docs/20-governance/22-adr/ADR-009-engine-attachment.md`, `docs/20-governance/22-adr/ADR-010-boot-entrypoint.md`
- Downstream runbook: `docs/20-governance/23-runbooks/workspaces-lifecycle.md`

## References
- `docs/20-governance/spine.md`
- `docs/20-governance/22-adr/ADR-007-workspace-isolation.md`
- `docs/20-governance/22-adr/ADR-008-connection-lifecycle.md`
