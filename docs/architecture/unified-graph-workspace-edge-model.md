# Unified Graph of Workspace + Edge Runtime Model (QG-2)

## Purpose

Define the canonical owner-side unified graph model that represents sovereign
workspace state together with governed distributed runtime state (edge, mesh,
delegation, transport, ingress, and coordination).

## Canonical thesis

The graph is unified in representation and owner-side in sovereignty.

Formula lock:

the graph unifies governed state; it does not erase adjudication boundaries.

## Graph scope

QG-2 graph model must include at least:

- workspace and owner runtime anchors
- edge runtime and peer identities
- mesh membership/awareness/legitimacy classes
- bindings and action points
- grants/policy snapshots/capability envelopes
- observation events, mediation outcomes, evidence classes
- transport/overlay/ingress operational state
- coordination coverage/overlap markers

## Entity families

### Sovereign and case anchors

- workspace
- owner runtime
- canonical case slice anchors

### Edge and mesh runtime

- source node / daemon instance
- mesh node / discovery / bootstrap
- coordination membership / peer awareness
- legitimacy / authority scope

### Binding and delegated operation

- source binding
- action point
- enrollment grant
- policy snapshot
- capability envelope

### Operational distributed state

- transport endpoint/path/channel
- owner remote ingress/session/decision
- overlay presence/target-association/path-mutation
- spool/retry/backlog and freshness/degradation markers

### Contributions and evidence

- source acquisition event
- source ingest outcome
- source evidence candidate
- owner acceptance/canonicalization lineage markers

## Relation families (baseline)

The graph must support relations equivalent to:

- workspace `governs` peer/runtime members
- peer/runtime `belongs_to` membership/coordination classes
- binding `exposes` action point
- grant/snapshot/envelope `scopes` runtime/binding/action-point behavior
- edge runtime `produces` event/outcome/evidence
- ingress `accepts|restricts|rejects` remote contribution attempts
- transport/overlay path `connects` owner and peer targets
- membership `covers` source scope and `overlaps_with` peers/scope markers
- canonical case slice `informed_by` governed accepted contributions

## Adjudication boundary model

Graph projection must preserve state-layer distinction:

- observed (raw/distributed/runtime-declared)
- accepted (ingress or owner processing accepted)
- canonicalized (owner final adjudication)
- degraded/restricted/hold/conflict states

Presence in graph does not imply canonical finality.

## Owner-side correlation

Distributed contributions are correlated owner-side in graph space:

- multiple peers can inform same workspace/case slice
- overlap/conflict markers can coexist with accepted/canonical states
- peer-to-peer consensus is not required for owner-side correlation

## Summary and projection model

QG-2 requires graph-derived summaries:

- workspace unified graph summary
- peer/edge graph summary
- delegation validity graph summary
- transport/overlay/ingress graph summary
- coordination/coverage/overlap graph summary

These summaries are reusable by CLI, qualification gates, and AI grounding.

## Guardrails

- Graph richness must not flatten sovereign authority.
- Transport/overlay graph signals must not imply legitimacy.
- Ingress accepted state must not be conflated with canonicalized truth.
- Coordination/awareness links must not imply distributed adjudication.

## Handoffs

QG-2 unlocks:

- QG-3 grounding over governed graph state
- DX-2 graph-aware inspect/watch surfaces
- QW-4/QW-6 real-flow and readiness evidence synthesis
