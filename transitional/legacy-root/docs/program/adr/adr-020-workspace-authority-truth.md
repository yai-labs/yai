---
role: support
status: accepted
audience: governance
owner_domain: program-adr
id: ADR-020
decision_id: ADR-020
supersedes: []
superseded_by: []
implements: []
evidenced_by: [docs/program/reports/runtime-convergence-report.md]
related: []
anchor: "#workspace-authority-and-truth-plane"
applies_to: 
effective_date: 2026-03-11
phase: SW-1
---
# ADR-020 - Workspace Authority and Truth Plane

## Context

RF/ER waves established subordinate edge runtime behavior, but the system still
needed an explicit lock on where final authority and canonical truth live.

Without this lock, edge-local operational state can be misread as autonomous
truth.

## Decision

Owner workspace runtime is locked as:

- sovereign authority plane
- canonical truth plane

Final authority, final adjudication, graph truth, database truth, canonical
case state, and canonical provenance binding are owner-side only.

## Edge Rule

Edge runtimes contribute governed observations, local outcomes, evidence
candidates, and operational state, but never autonomous canonical truth.

## Consequences

### Positive

- clear boundary for SW-2/SW-3 policy distribution and validity
- stable foundation for mesh and WAN waves without sovereignty drift
- cleaner qualification criteria for owner-side final adjudication

### Negative

- stricter requirements on owner acceptance/canonicalization paths
- more explicit modeling burden for edge-vs-owner state classes

## References

- `docs/architecture/workspace-authority-and-truth-plane-model.md`
- `docs/architecture/runtime-model.md`
- `docs/architecture/canonical-data-plane-model.md`
