# Source Plane Read Model

Status: active
Owner: runtime
Effective date: 2026-03-10

## Purpose

Define the owner-side read model for source-plane entities after YD-6.
This document covers inspect/query and graph materialization visibility for
source-plane records persisted by owner ingest.

## Scope

YD-6 extends read surfaces for:

- `source_node`
- `source_daemon_instance`
- `source_binding`
- `source_asset`
- `source_acquisition_event`
- `source_evidence_candidate`
- `source_owner_link`
- `workspace_peer_membership`

Read model is owner-side only. `yai-daemon` does not host canonical graph truth.

## Query and inspect surfaces

`yai.workspace.query source` returns `source_plane_summary` with:

- class counts (`source_*_count`)
- peer orchestration counts (`workspace_peer_membership_count`)
- source record tails (`source_nodes`, `source_bindings`, `source_assets`, ...)
- peer orchestration tails (`workspace_peer_memberships`)
- graph counters (`source_graph_node_count`, `source_graph_edge_count`)
- DB-first read-path metadata
- workspace graph summary context

Existing graph workspace summary now includes source-plane counters:

- `source_graph_node_count`
- `source_graph_edge_count`

## Graph projection baseline

Owner-side graph materialization projects source-plane records into node/edge
relations with canonical operational semantics.

Nodes:

- `source_node`
- `source_daemon_instance`
- `source_binding`
- `source_asset`
- `source_acquisition_event`
- `source_evidence_candidate`
- `workspace_peer_membership`
- owner workspace anchor

Edges:

- `attached_to` (`source_node -> owner_workspace`)
- `runs_on` (`source_daemon_instance -> source_node`)
- `bound_on` (`source_binding -> source_node`)
- `targets_workspace` (`source_binding -> owner_workspace`)
- `discovered_via` (`source_asset -> source_binding`)
- `observed` (`source_acquisition_event -> source_asset`)
- `emitted_by` (`source_acquisition_event -> source_node`)
- `derived_from` (`source_evidence_candidate -> source_acquisition_event`)
- `member_of_workspace` (`workspace_peer_membership -> owner_workspace`)
- `membership_source_node` (`workspace_peer_membership -> source_node`)
- `membership_binding` (`workspace_peer_membership -> source_binding`)

## Semantics guardrails

- `source_asset` is not owner canonical artifact truth.
- `source_evidence_candidate` is not final owner evidence truth.
- Source-plane graph projection is materialized by owner runtime only.
- No local source-node graph is treated as canonical owner graph.
- `workspace_peer_membership` is coordination metadata, not authority finality.

## Verification baseline

Representative integration path:

- `tests/integration/source_plane/source_plane_read_model_v1.sh`

The test validates persistence -> query -> graph chain:

1. source enroll/attach/emit/status through owner ingest
2. `yai.workspace.query source` summary counts and graph counters
3. `yai.workspace.graph.workspace` source graph counters

## Known YD-6 limits

- v1 read model is summary-first; advanced filtering is deferred.
- graph projection keeps a compact edge set; dense semantics deferred.
- CLI ergonomic formatting for each source subfamily is deferred to CLI waves.

## References

- `docs/architecture/source-plane-model.md`
- `docs/architecture/source-owner-ingest-model.md`
- `docs/architecture/daemon-local-runtime-model.md`
