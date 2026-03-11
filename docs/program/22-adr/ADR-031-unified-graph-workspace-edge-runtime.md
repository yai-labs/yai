# ADR-031 — Unified Graph of Workspace + Edge Runtime (QG-2)

## Status

Accepted

## Context

QG-1 introduced canonical inspect/query surfaces, but distributed runtime state
is still fragmented unless represented in a unified owner-side relational graph.

## Decision

YAI adopts a unified governed graph model that represents workspace sovereignty
and distributed runtime contribution/state in one owner-side graph space:

- workspace/case anchors remain sovereign
- edge/mesh/delegation/transport/ingress classes are graph-represented
- relation semantics explicitly preserve adjudication layers

Boundary lock:

graph unification does not imply distributed sovereign truth or automatic final
adjudication.

## Consequences

- Graph summaries can explain distributed runtime behavior coherently.
- QG-3 can ground AI over governed relational state instead of raw blobs.
- DX/QW surfaces can reuse stable graph projection semantics.

## Non-goals

- graph-as-consensus across peers
- conflating observed/accepted/canonicalized states
- treating transport or awareness links as authority equivalence
