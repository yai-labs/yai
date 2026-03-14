---
role: support
status: accepted
audience: governance
owner_domain: program-adr
id: ADR-024
decision_id: ADR-024
supersedes: []
superseded_by: []
implements: []
evidenced_by: [docs/program/reports/runtime-convergence-report.md]
related: []
---
# ADR-024 — Mesh Discovery Foundation (MF-1)

## Status

Accepted

## Context

MF-A1 established YAI as mesh-native in topology and sovereign in authority.
The next required slice is a formal Mesh Discovery Plane so nodes can be
identified, advertised, and bootstrap-discovered without collapsing discovery
into trust or authority.

## Decision

YAI adopts an explicit governed Mesh Discovery Plane with:

- canonical node advertisement model
- owner discovery and peer discovery as separate concerns
- bootstrap discovery descriptor model
- scope-aware visibility semantics

Boundary locks:

- discoverability/visibility do not imply enrollment
- discoverability/visibility do not imply trust
- discovery does not transfer sovereign authority

## Consequences

- MF-2 can build coordination on explicit discovery state.
- MF-3 can bind trust/enrollment on top of discovery without redesign.
- SDK/CLI discovery surfaces can be modeled without authority ambiguity.
- MT waves can integrate overlay/WAN reachability changes into discovery scope.

## Non-goals

- trust finalization in discovery plane
- grant/capability issuance in discovery plane
- peer-equal sovereign mesh model
