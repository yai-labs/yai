# Owner Remote Peer Ingress Model (MT-2)

## Purpose

Define the canonical owner-side remote ingress boundary for peer traffic over
the secure overlay transport plane.

MT-2 turns transport availability into a governed owner runtime boundary:

- transport path enables presentation,
- ingress policy decides acceptance/restriction/rejection,
- canonical truth remains owner-side and downstream of ingress.

## Canonical thesis

Owner remote peer ingress is a hardened and governed boundary, not a generic
open endpoint.

Formula lock:

transport reachability enables presentation; ingress governance decides
acceptance.

## Boundary model

The ingress boundary evaluates remote peer attempts against:

- legitimacy/enrollment state
- delegated scope and contribution class
- delegated material validity state (valid/stale/expired/revoked)
- ingress readiness and restriction mode
- transport/path/channel health context

## Non-equivalences (hard locks)

- reachable != accepted
- connected != enrolled
- endpoint-alive != authorized contribution scope
- ingress-accepted != canonical truth

## Ingress hardening semantics

Ingress hardening is architectural, not only socket-level:

- peer/context recognition
- enrollment/trust coherence checks
- delegated scope and validity checks
- contribution-class filtering
- degraded/restricted acceptance modes
- explicit rejection/defer reasons

## Ingress states

Canonical ingress state vocabulary (baseline):

- `ingress_ready`
- `ingress_degraded`
- `ingress_restricted`
- `ingress_unavailable`
- `ingress_pending_trust_refresh`
- `ingress_limited_contribution_classes`

These states are operational and inspectable owner-side.

## Contribution-class acceptance model

Ingress decisions are class-aware and can differ by contribution type:

- observation/event contributions
- evidence candidate contributions
- local mediation outcomes
- edge runtime health/freshness state
- spool-delayed/replay deliveries
- refresh/revalidation-related artifacts
- coordination signals (when permitted)

Acceptance scope can be narrower than transport connectivity.

## Acceptance to canonicalization chain

Correct chain:

1. peer reaches ingress over secure transport
2. ingress evaluates and decides accept/restrict/reject/defer
3. accepted material enters owner runtime processing
4. canonicalization/adjudication remains owner sovereign process

Ingress acceptance alone never grants canonical truth.

## Compatibility with stale/revoked/degraded peers

MT-2 supports cases such as:

- reachable but not fully trusted peer
- stale delegated material with restricted ingress
- expired/revoked scope with rejection or observe-only acceptance
- reconnecting peer replaying spool under bounded acceptance
- suspended peer allowed health-state reporting only

## Owner inspect/ops visibility baseline

Owner runtime must expose ingress-operational visibility for:

- presenting peer identity/context
- ingress decision state
- restriction/degradation reason
- accepted contribution classes
- rejection/defer reasons
- linked transport path/channel state

## Cross-repo handoff

MT-2 anchors:

- MT-3 overlay integration implementation details
- QG inspect/query surfaces for ingress visibility
- QW peering/WAN qualification scenarios

Authority remains governed by SW/MF locks.

Overlay-native deployment integration is defined in:
`docs/architecture/overlay-integration-model.md`.
