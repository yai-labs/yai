# Daemon Architecture Refoundation Model (YD-1)

## Purpose

Lock the canonical owner/daemon topology before feature slices.

This document is the YD-1 refoundation anchor and prevents later slices from
re-deciding foundational boundaries.

## Canonical Topology

- Owner runtime: `yai`
- Edge runtime binary: `yai-daemon` (subordinate edge runtime)
- Topology: `distributed acquisition / centralized control`

`yai` remains the only canonical workspace truth and governance runtime.

`yai-daemon` is a subordinate edge runtime that can observe, mediate delegated
local actions, spool, retry and deliver source-plane units, but it is never a
second workspace or policy truth owner.

## Boundary Lock

### Owner (`yai`)

- canonical workspace lifecycle/binding truth
- canonical authority/evidence/enforcement outcomes
- canonical persistence and graph materialization
- canonical read/query surfaces

### Edge (`yai-daemon`)

- source-local acquisition activity and delivery attempts
- delegated local action mediation and enforcement (owner-scoped only)
- local spool/retry operational state
- health/status signals for owner-side mediation
- no canonical graph/workspace/policy/conflict truth

## Sovereignty Hierarchy

1. workspace global policy plane (`yai`, sovereign)
2. delegated edge policy plane (owner-issued snapshots/grants/envelopes)
3. edge execution/observation plane (`yai-daemon`, subordinate)

## Exec Role Lock

`exec` is the active source-plane mediation layer between edge inputs and owner
runtime truth:

- transport adaptation
- source-owner routing handoff
- runtime/gate preflight
- owner-side ingest mediation

`exec` is not owner truth and does not replace `core`.

## Naming Lock

- binary: `yai` (owner runtime host)
- binary: `yai-daemon` (subordinate edge runtime)
- node term: `source node` (edge runtime host)
- control term: `owner runtime`

Deprecated/ambiguous naming is out of contract for YD-1 forward slices.

## Out of Scope (YD-1)

- full daemon runtime behavior
- secure peering implementation
- complete multi-peer orchestration
- full CLI/SDK/LAW product surfaces

YD-1 provides shape and guardrails, not full distributed behavior.

## Downstream Slices Unblocked

- YD-2: daemon binary baseline
- YD-3: source-plane model classes/contracts
- YD-4: owner ingest bridge
- YD-7: exec verticalization

## References

- `docs/program/22-adr/ADR-013-distributed-acquisition-centralized-control.md`
- `docs/program/22-adr/ADR-015-daemon-architecture-refoundation-slice.md`
- `docs/architecture/exec-source-plane-role.md`
