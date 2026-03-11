# Owner Peer Registry and Coordination Model (OP-2)

Status: active
Owner: runtime
Effective date: 2026-03-11

## Purpose

Define the owner-side runtime registry and coordination baseline for multiple
peers attached to one workspace.

OP-2 turns multi-peer orchestration from model-only (OP-1) into runtime
mechanics: owner keeps a live workspace-aware peer set and derives scheduling
signals from per-peer activity/freshness/backlog.

## Runtime components

### 1) Peer registry

Owner runtime keeps an in-memory registry keyed by:

- workspace
- source node
- source binding

Registry entry stores:

- daemon instance
- role/scope
- peer state
- backlog counters
- coverage and overlap signals
- last seen / last activity / updated timestamps

Implementation anchors:

- `include/yai/exec/peer_registry.h`
- `lib/exec/runtime/peer_registry.c`

### 2) Coordination updates

Registry updates are fed by source-plane operations:

- `attach`: creates baseline membership coordination state
- `status`: updates health/freshness/backlog signals
- `emit`: records activity and backlog pressure evolution

### 3) Workspace coordination summary

Owner exposes workspace coordination summary with:

- peer counts by effective state (`ready`, `degraded`, `disconnected`, `stale`)
- aggregated backlog (`queued`, `retry_due`, `failed`)
- scheduling baseline state:
  - `nominal`
  - `backlog_pressure`
  - `attention_required`
- per-peer runtime coordination rows

## Freshness semantics (baseline)

Derived from `last_seen_epoch`:

- `fresh`: <= 30s
- `stale`: <= 120s
- `disconnected`: > 120s

These thresholds are runtime baseline constants for OP-2 and can be policy-tuned
later.

## Scheduling baseline semantics

OP-2 does not implement a full scheduler.
It computes owner-centric coordination state:

- `nominal`: no disconnect/failure pressure
- `backlog_pressure`: retry/backlog/staleness pressure exists
- `attention_required`: disconnected peers or failed backlog present

This is enough for OP-3/OP-4 to build read and conflict layers without
defining a distributed scheduler.

## Boundaries

- Registry is runtime-operational, not law engine.
- Registry is not the graph projection itself.
- Coordination baseline is not conflict resolution closure.

MF-2 compatibility:
- this owner peer registry is the concrete owner-anchored substrate of the Mesh
  Coordination Plane;
- peer awareness, coverage/overlap, and ordering/replay signals remain
  coordination-plane semantics and do not transfer sovereign authority.

## References

- `docs/architecture/workspace-peer-orchestration-model.md`
- `docs/architecture/source-plane-read-model.md`
- `docs/program/milestone-packs/runtime-baselines/workspace-peer-coordination-baseline.md`
