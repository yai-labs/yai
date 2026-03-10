---
id: RB-DP-11-GRAPH-MATERIALIZATION
title: Graph Materialization from Runtime Records
status: draft
owner: runtime
effective_date: 2026-03-10
depends_on:
  - RB-DP-10-ENFORCEMENT-RECORD-CLOSURE
  - RB-BRAIN-MEMORY-GRAPH-SINKS
tags:
  - data-plane
  - graph
  - materialization
  - runtime
---

# DP-11 — Graph Materialization Hooks from Runtime Decisions

## 1) Purpose
Materialize persistent graph truth from canonical runtime records.

Graph entries are now record-driven from persisted sources, not cache-driven:
- runtime events
- decision records
- evidence records
- governance refs
- authority refs
- artifact refs
- workspace refs

## 2) Canonical source records
Materialization hooks consume runtime-persisted refs emitted by DP-10 closure:
- `event_ref`
- `decision_ref`
- `evidence_ref`
- governance refs CSV (first canonical token)
- `authority_ref`
- `artifact_ref`
- workspace id/scope.

## 3) Canonical node classes (baseline)
- `workspace_node`
- `governance_object_node`
- `decision_node`
- `evidence_node`
- `authority_node`
- `artifact_node`
- `runtime_episode_node`

Persisted as `yai.brain_graph_node.v1` rows in:
- `~/.yai/run/<ws>/brain/graph/persistent-nodes.v1.ndjson`

## 4) Canonical edge classes (baseline)
- `decision_in_workspace`
- `decision_under_governance`
- `decision_under_authority`
- `decision_on_artifact`
- `evidence_for_decision`
- `artifact_governed_by`
- `workspace_uses_governance`
- `episode_yielded_decision`

Persisted as `yai.brain_graph_edge.v1` rows in:
- `~/.yai/run/<ws>/brain/graph/persistent-edges.v1.ndjson`

## 5) Hook implementation and ownership
Primary writer hook:
- `yai_mind_storage_bridge_resolution_hook(...)`
  in `lib/brain/memory/storage_bridge.c`

Runtime invokes the hook from resolution snapshot path after event/evidence,
governance, authority, artifact persistence is emitted.

## 6) Idempotency discipline
Node/edge refs are deterministic per runtime record context.
Append path is uniqueness-aware (`graph_node_ref` / `graph_edge_ref`), preventing
duplicate semantic rows for repeated refs.

## 7) Lineage and provenance
Node records include:
- `origin_domain`
- `source_record_ref`
- runtime refs (`event_ref`, `decision_ref`, `evidence_ref`)
- linkage refs (`governance_ref`, `authority_ref`, `artifact_ref`)

Graph index includes:
- `materialization_mode: runtime_record_driven`
- `materialization_status: complete`
- declared source record set
- declared node/edge class sets.

## 8) Graph truth vs transient cognition
DP-11 writes graph truth to BR-3 graph sink.
Transient cognition stays BR-4 and explicitly non-authoritative.

This keeps: graph sovereignty != graph heat.

## 9) Failure semantics
If graph materialization fails:
- runtime decision/evidence truth remains primary and intact,
- brain hook returns degraded status,
- failure reason remains inspectable (`brain_sink_degraded` path).

## 10) Verification
Integration smoke:
- `tests/integration/workspace_lifecycle/workspace_graph_materialization_hooks_dp11_v1.sh`

Checks include:
- node/edge class coverage,
- record-driven provenance fields,
- graph index materialization markers,
- read-surface no-write behavior,
- graph truth still separated from transient authority semantics.

Matrix integration:
- `tests/integration/workspace_lifecycle/workspace_final_demo_matrix_v1.sh`

## 11) Handoff
DP-11 enables:
- DP-12 DB-first read path cutover (readers can trust canonical persisted graph refs)
- DP-16 graph baseline read/summary surfaces over typed node/edge graph truth.
