# ADR-028 — Owner Remote Peer Ingress (MT-2)

## Status

Accepted

## Context

MT-1 defined secure overlay transport as a distinct plane. A dedicated owner
ingress boundary is required so transport reachability does not become implicit
acceptance.

## Decision

YAI adopts an explicit owner remote peer ingress boundary with governed
acceptance logic:

- transport only enables remote presentation
- ingress evaluates legitimacy/scope/validity/contribution class
- ingress can accept, restrict, defer, degrade, or reject
- ingress acceptance is not canonical truth

## Consequences

- MT-3 can integrate concrete overlay operation on a hardened owner boundary.
- Remote replay/stale/revoked cases can be handled with explicit ingress modes.
- Query/inspect/runbooks can expose ingress readiness/restriction/rejection
  state coherently.

## Non-goals

- auto-accept based only on reachability/connectivity
- flattening trust/enrollment decisions into transport layer
- treating ingress acceptance as final owner adjudication
