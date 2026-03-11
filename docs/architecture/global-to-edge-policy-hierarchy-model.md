# Global-to-Edge Policy Hierarchy Model (RF-0.2)

Status: active  
Owner: runtime  
Effective date: 2026-03-11

## Purpose

Lock authority hierarchy between owner policy sovereignty and edge delegated
execution behavior.

## Three-Plane Hierarchy

1. **Workspace global policy plane** (owner sovereign, `yai`)
2. **Delegated edge policy plane** (owner-issued snapshots/grants/capability envelopes)
3. **Edge execution/observation plane** (`yai-daemon`, subordinate runtime)

## Sovereignty Rules

- Policy truth is owner-side only.
- Graph truth is owner-side only.
- Conflict truth is owner-side only.
- DB/canonical state truth is owner-side only.

Edge delegated state is never sovereign truth.

## Delegation Model

The owner can distribute bounded operational authority to edge runtime through:

- policy snapshots
- grants
- capability envelopes

Delegation properties:

- scope-limited
- revocable
- temporally bounded
- non-sovereign

## Precedence and Containment

- Global workspace policy always overrides edge delegated state.
- Edge runtime must stay inside granted scope.
- Missing/stale delegated policy reduces autonomy; it never increases autonomy.
- Ambiguity/conflict falls back to owner authority precedence.

## Non-Equivalence Rule

Delegated execution is not sovereign policy ownership.

That means:

- grant != sovereignty transfer
- capability envelope != authority creation
- snapshot cache != policy truth
- local acceptance != owner final validation

## Foundations for Next Slices

RF-0.2 prepares semantics for:

- grant issuance/refresh/revoke
- capability limitation and temporal validity
- stale/degraded/disconnected edge behavior policy
- delegated edge enforcement model

without claiming full implementation in this tranche.

RF-0.3 consumes this hierarchy and defines enforcement levels/outcomes.

## References

- `docs/architecture/source-plane-model-refoundation-rf01.md`
- `docs/architecture/runtime-model.md`
- `docs/architecture/distributed-acquisition-plane-model.md`
- `docs/program/22-adr/ADR-016-global-to-edge-policy-hierarchy-lock.md`
