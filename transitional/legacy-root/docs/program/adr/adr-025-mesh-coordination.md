---
role: support
status: accepted
audience: governance
owner_domain: program-adr
id: ADR-025
decision_id: ADR-025
supersedes: []
superseded_by: []
implements: []
evidenced_by: [docs/program/reports/runtime-convergence-report.md]
related: []
---
# ADR-025 — Mesh Coordination Foundation (MF-2)

## Status

Accepted

## Context

MF-A1 and MF-1 established governed mesh topology and discovery. The next
required slice is explicit coordination semantics so multiple peers can operate
as governed workspace members with observable distributed state.

## Decision

YAI adopts an explicit Mesh Coordination Plane with:

- governed mesh membership state
- owner-anchored peer registry
- peer awareness metadata model
- baseline coordination semantics for coverage/overlap, scheduling, ordering,
  replay, and distributed conflict pressure

Boundary lock:

- coordination metadata and awareness do not transfer sovereign authority
- coordination does not move canonical truth or final conflict adjudication from
  owner workspace runtime

## Consequences

- MF-3 can enforce sovereign authority boundaries with explicit coordination
  substrate.
- QG query/graph surfaces can represent distributed operation coherently.
- QW scale and WAN qualification can validate peer coordination realism.

## Non-goals

- distributed sovereign adjudication across peers
- replacing owner authority with peer-majority coordination logic
- conflating discovery visibility with coordinated membership acceptance
