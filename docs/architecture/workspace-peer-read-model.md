# Workspace Peer Read Model (OP-3)

## Purpose

Define the owner-side read model for one canonical workspace fed by multiple peers.

The runtime already coordinates peer membership and activity (OP-2). This slice makes
that coordination observable through query and graph surfaces.

## Read Surfaces

### 1) `yai.workspace.query source`

Workspace-level multi-peer source summary.

Includes:
- source-plane record counts
- workspace peer membership counts
- owner coordination summary (`coordination`)
- graph source counts

`coordination` now exposes:
- peer states (`ready`, `degraded`, `disconnected`, `stale`)
- backlog totals (`queued`, `retry_due`, `failed`)
- scheduling baseline state
- coverage summary (`scope_count`, `distinct`, `overlap`, `gap`)
- per-peer rows with freshness/coverage/overlap

### 2) `yai.workspace.query source.peer`

Peer inspect surface in workspace context.

Returns per-peer runtime rows with:
- `source_node_id`, `source_binding_id`, `daemon_instance_id`
- `peer_role`, `peer_scope`
- effective state and freshness
- backlog counters
- `coverage_ref`, `overlap_state`
- `last_seen_epoch`, `last_activity_epoch`

### 3) `yai.workspace.query source.coverage`

Workspace coverage summary across attached peers.

Returns:
- coverage scope count
- distinct coverage count
- overlap count
- gap count

### 4) `yai.workspace.query source.conflicts`

Baseline conflict/order/replay view from persisted ingest outcomes.

Returns recent rows with:
- classification (`clean`, `duplicate_replay`, `overlap_ambiguous`, ...)
- handling action (`accept`, `accept_with_flag`, `review_stub`)
- ordering and replay status
- review-required markers

## Graph Extension

`workspace_peer_membership` records now materialize coverage topology nodes/edges:

- node class: `source_scope`
- edge: `membership_covers_scope` (`workspace_peer_membership -> source_scope`)
- edge: `binding_scope` (`source_binding -> source_scope`)
- edge: `overlap_on_scope` (`source_node -> source_scope`) when overlap is signaled

This keeps graph topology readable without introducing a conflict engine in OP-3.

## Semantics

- Workspace remains canonical and unique.
- Peers are members of workspace source-plane orchestration, not mini-workspaces.
- Coverage/overlap visibility is baseline observability, not final conflict resolution.
- Health and backlog are operational signals, not authority decisions.

## Out of Scope

- conflict resolution engine
- replay ordering resolution
- advanced graph analytics
- UI dashboards

These remain for OP-4 and later slices.

## References

- `docs/architecture/workspace-peer-orchestration-model.md`
- `docs/architecture/owner-peer-registry-and-coordination-model.md`
- `docs/architecture/source-plane-read-model.md`
