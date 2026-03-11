# Overlay Integration Model (MT-3)

## Purpose

Define how private secure overlay connectivity becomes a native part of YAI
distributed deployment for owner and peer runtimes.

MT-3 closes transport wave by integrating:

- MT-1 secure overlay transport plane
- MT-2 owner remote ingress boundary

into one deployable overlay-aware runtime model.

## Canonical thesis

Overlay integration is deployment-native runtime composition, not a network-only
add-on.

Formula lock:

overlay presence is transport identity, not sovereign legitimacy.

## Overlay-native deployment model

Canonical non-LAN baseline:

- owner runtime present on private overlay
- peer runtimes present on private overlay
- owner/peer endpoint association via overlay-private addressing
- path/state observability under changing reachability
- owner ingress governance preserved for all remote paths

This model supports remote operation without flattening sovereignty.

## Identity and association model

MT-3 relates four identities without conflation:

- mesh node identity
- overlay transport identity
- remote endpoint identity
- authority/legitimacy identity

Boundary locks:

- overlay endpoint != enrollment state
- transport identity != trust legitimacy
- path reachability != delegated scope validity

## Runtime targeting integration

Overlay-aware targeting must represent:

- owner remote target
- peer remote target
- endpoint association to node identity
- path/readiness/degradation state
- endpoint change/mutation continuity

Targeting is operational; authority remains owner-governed.

## Owner ingress continuity

Overlay does not bypass MT-2 ingress governance:

- remote peers present over overlay
- owner ingress evaluates legitimacy/scope/validity/contribution class
- acceptance can be restricted/deferred/rejected
- canonicalization remains owner-side adjudication

## Overlay-aware runtime state

Baseline vocabulary for integrated state:

- `overlay_connected`
- `overlay_degraded`
- `path_stale`
- `endpoint_changed`
- `transport_rebind_required`
- `peer_reachable_restricted`
- `owner_reachable_refresh_pending`
- `private_path_unavailable_local_runtime_active`

## Mutable reachability and endpoint mutation

MT-3 explicitly supports:

- peer network moves with retained node identity
- endpoint/path mutation with rebind requirements
- temporary overlay loss while local edge runtime continues constrained
- overlay restoration with resumed refresh/revalidation

## Cross-repo alignment contract

### `yai`

- model overlay descriptors/associations/path state as first-class runtime classes
- keep strict boundary between overlay state and authority/legitimacy

### `yai-sdk`

- expose overlay-aware owner/peer locators and remote target associations
- separate transport mutation/readiness from enrollment/trust semantics

### `yai-cli`

- inspect/list/watch overlay state, target association, ingress/transport condition
- keep operator semantics layered and non-ambiguous

### `yai-law`

- preserve boundaries between overlay presence and sovereign legitimacy/authority
- prevent governance drift from connectivity-only interpretations

## Handoffs

MT-3 unlocks:

- QG read/query surfaces for overlay-aware runtime visibility
- DX SDK/CLI targeting surfaces
- QW secure peering and WAN resilience qualification waves
