# RF-0.1 Source Plane Model Refoundation

Status: active  
Owner: runtime  
Effective date: 2026-03-11

## Canonical Statement

`yai-daemon` is the canonical standalone subordinate edge runtime of the YAI
architecture: it performs local asset/process observation, delegated action
mediation, local spool/retry/health handling, and edge-side execution under
owner-issued policy scope, while `yai` remains the sole sovereign workspace
runtime and source of truth for policy, graph, conflict resolution and
canonical state.

## Hierarchy Lock

1. Workspace global policy plane (owner sovereign)
2. Delegated edge policy plane (owner-issued snapshots/grants/envelopes)
3. Edge execution/observation plane (subordinate runtime behavior)

## Edge Runtime Allowed (owner-scoped only)

- local observation (assets, process-local signals baseline)
- local action mediation points
- delegated local enforcement actions (`allow` / `block` / `hold` / `execute`)
- local spool/retry/reconnect/health management

## Edge Runtime Forbidden

- owning canonical workspace truth
- owning global policy truth
- owning graph truth
- owning conflict-resolution truth
- independent sovereignty from owner authority plane

## Notes

This refoundation is semantic/model alignment only. It does not claim:

- full delegated enforcement implementation
- full policy distribution implementation
- full mesh/federation behavior

## References

- `docs/architecture/runtime-model.md`
- `docs/architecture/distributed-acquisition-plane-model.md`
- `docs/architecture/source-plane-model.md`
- `docs/architecture/daemon-local-runtime-model.md`
- `docs/architecture/exec-source-plane-role.md`
