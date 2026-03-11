# ADR-029 — Overlay Integration (MT-3)

## Status

Accepted

## Context

MT-1 established secure transport plane boundaries and MT-2 established owner
remote ingress governance. A deployment-native integration model is required so
remote operation is first-class and not an ad-hoc networking exception.

## Decision

YAI adopts overlay integration as native distributed deployment model:

- owner/peer runtime targeting is overlay-aware
- overlay endpoint/path state is runtime-visible and mutable over time
- node identity is associated with transport identity without conflating trust
- owner ingress governance remains mandatory on overlay paths

Boundary lock:

overlay presence/reachability never replaces enrollment, trust, delegated
scope validity, ingress governance, or workspace sovereignty.

## Consequences

- DX and QG surfaces can model remote targeting and overlay transitions
  coherently.
- QW secure peering/WAN resilience waves can validate endpoint mutation and
  path degradation/recovery scenarios.
- MT transport stack remains layered under sovereign authority semantics.

## Non-goals

- inferring legitimacy from overlay address visibility
- bypassing ingress policy because transport is healthy
- treating transport identity as canonical authority state
