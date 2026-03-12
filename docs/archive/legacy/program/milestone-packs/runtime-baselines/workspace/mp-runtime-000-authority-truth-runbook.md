# SW-1 Workspace Authority and Truth Plane Baseline

Status: active  
Owner: runtime  
Effective date: 2026-03-11

## Purpose

Operationally enforce and communicate that final authority and canonical truth
are owner-side only.

## Owner-side canonical classes

- global policy truth
- graph truth
- database truth
- canonical case state
- final conflict adjudication
- canonical provenance binding

## Edge contribution classes

- governed observation
- local delegated outcomes
- evidence candidates
- operational runtime state (health/freshness/spool/retry/connectivity)

These classes are input signals and operational context, not autonomous
canonical truth.

## Operator guardrails

- do not treat edge-local spool/snapshot/outcome as final case truth
- treat owner acceptance/persistence as canonicalization boundary
- treat owner-side conflict decision as final adjudication

## References

- `docs/architecture/workspace-authority-and-truth-plane-model.md`
- `docs/program/adr/adr-workspace-020-workspace-authority-and-truth-plane.md`
