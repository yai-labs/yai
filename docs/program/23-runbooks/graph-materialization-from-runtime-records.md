---
id: RB-DP-11-GRAPH-MATERIALIZATION
title: Graph Materialization from Runtime Records
status: active
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

Graph entries are record-driven from persisted sources, not cache-driven:
- runtime events,
- decision records,
- evidence records,
- governance refs,
- authority refs,
- artifact refs,
- workspace refs.

## 2) Canonical source records

Materialization hooks consume runtime-persisted refs emitted by DP-10 closure:
- `event_ref`
- `decision_ref`
- `evidence_ref`
- governance refs CSV (first canonical token)
- `authority_ref`
- `artifact_ref`
- workspace id/scope

## 3) Canonical node classes (baseline)

- `workspace_node`
- `governance_object_node`
- `decision_node`
- `evidence_node`
- `authority_node`
- `artifact_node`
- `runtime_episode_node`

Persisted as `yai.graph_node.v1` rows in:
- `~/.yai/run/<ws>/runtime/graph/persistent-nodes.v1.ndjson`

## 4) Canonical edge classes (baseline)

- `decision_in_workspace`
- `decision_under_governance`
- `decision_under_authority`
- `decision_on_artifact`
- `evidence_for_decision`
- `artifact_governed_by`
- `workspace_uses_governance`
- `episode_yielded_decision`

Persisted as `yai.graph_edge.v1` rows in:
- `~/.yai/run/<ws>/runtime/graph/persistent-edges.v1.ndjson`

## 5) Hook implementation and ownership

Primary writer path:
- runtime resolution snapshot writes persisted records,
- graph materialization hook in graph/data binding flow writes node/edge stores,
- workspace summary surfaces are refreshed with latest graph refs.

## 6) Idempotency discipline

Node/edge refs are deterministic per runtime record context.
Append path is uniqueness-aware (`graph_node_ref` / `graph_edge_ref`) to prevent
semantic duplicates for repeated refs.

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
- declared node/edge class sets

## 8) Graph truth vs transient knowledge state

DP-11 writes graph truth to runtime graph sink.
Transient state stays non-authoritative under runtime transient sink.

This preserves: authoritative graph truth != transient activation context.

## 9) Failure semantics

If graph materialization fails:
- runtime decision/evidence truth remains primary and intact,
- runtime status reports degraded graph readiness,
- failure reason stays inspectable in workspace/status surfaces.

## 10) Verification

Integration smoke:
- `tests/integration/workspace/workspace_graph_materialization_hooks.sh`

Checks include:
- node/edge class coverage,
- record-driven provenance fields,
- graph index materialization markers,
- read-surface no-write behavior,
- graph truth separated from transient state authority semantics.

Matrix integration:
- `tests/integration/workspace/workspace_demo_matrix.sh`

## 11) Handoff

DP-11 enables:
- DP-12 DB-first read path cutover,
- DP-16 graph baseline read/summary surfaces over typed graph truth.

## 12) Source-plane extension note (YD-3 -> YD-6)

Source-plane entity classes are defined in YD-3 (`source_node`,
`source_daemon_instance`, `source_binding`, `source_asset`,
`source_acquisition_event`, `source_evidence_candidate`,
`source_owner_link`).

YD-3 does not materialize these graph nodes/edges yet.
YD-6 extends graph materialization/read surfaces using these canonical classes.
