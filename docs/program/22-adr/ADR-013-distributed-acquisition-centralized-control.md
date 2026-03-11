---
id: ADR-013
status: accepted
effective_date: 2026-03-10
supersedes: []
applies_to:
  runbook: docs/program/23-runbooks/engine-attach.md
  phase: YD-1
  anchor: "#source-plane-refoundation"
law_refs:
  - ../law/foundation/invariants/I-003-governance.md
  - ../law/foundation/invariants/I-006-external-effect-boundary.md
  - ../law/foundation/boundaries/L1-kernel.md
  - ../law/foundation/boundaries/L2-engine.md
---
# ADR-013 - Distributed Acquisition Plane, Centralized Control Plane

## Context

The unified runtime has one owner runtime (`yai`) and workspace-first truth.
The program now introduces multi-machine source acquisition without introducing
runtime federation or secondary truth owners.

## Decision

YAI adopts this v1 topology model:

- distributed acquisition plane on source nodes via standalone subordinate `yai-daemon`
- centralized control plane on owner runtime `yai`

`yai` remains canonical owner/runtime source of truth for:

- workspace lifecycle and binding
- authority/evidence/enforcement final outcomes
- canonical persistence and graph truth

`yai-daemon` is a subordinate edge runtime and is explicitly not an owner
runtime or independent sovereign policy/truth authority.

`exec` is the active runtime layer for owner/daemon mediation (transport,
routing handoff, and acquisition gating).

## Rationale

- Enables multi-machine acquisition in trusted-network pre-pilot conditions.
- Preserves one authority and one canonical truth path.
- Avoids federated-runtime ambiguity during v1 rollout.
- Provides a clean architecture contract for YD-2..YD-7 implementation slices.

## Consequences

### Positive

- Clear owner/daemon boundary and naming (`yai-daemon` canonical).
- No confusion between acquisition distribution and control-plane ownership.
- Deterministic upgrade path for transport/ingest verticalization.

### Negative

- Edge autonomy is intentionally limited in v1.
- Delegated edge execution is always owner-scoped.
- Federation-style capabilities are explicitly deferred.
- Requires explicit docs/runtime alignment to avoid local-only assumptions.

## Non-goals

- Runtime federation or peer-owner mesh.
- Edge-local final authority/evidence/enforcement decisions.
- Edge-local canonical graph/workspace truth.
- Network security hardening design (WireGuard/VPN) in this tranche.

## Traceability

- Architecture model:
  - `docs/architecture/distributed-acquisition-plane-model.md`
- Exec role baseline:
  - `docs/architecture/exec-source-plane-role.md`
- Anchoring ADR:
  - `docs/program/22-adr/ADR-001-single-runtime.md`

## Status

Accepted and active.
