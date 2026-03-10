---
id: RB-DP-8-DATA-SURFACES
title: CLI/SDK Data Surfaces and Operator Query Model
status: draft
owner: runtime
effective_date: 2026-03-10
depends_on:
  - RB-DATA-PLANE
  - RB-EVENT-EVIDENCE-PERSISTENCE
  - RB-GOVERNANCE-COMPLIANCE-PERSISTENCE
  - RB-AUTHORITY-ARTIFACT-PERSISTENCE
  - RB-BRAIN-MEMORY-GRAPH-SINKS
tags:
  - data-plane
  - operator-query
  - cli
  - sdk
  - runtime
---

# DP-8 — CLI/SDK Data Surfaces and Operator Query Model

## 1) Purpose
Expose canonical runtime-mediated data surfaces over live Data Plane objects.

The public model is object-centric and command-centric:
- `cli -> sdk -> yai`
- no direct backend coupling (`LMDB` / `DuckDB` / `Redis` are internal roles)
- no filesystem-path contract as operator UX.

## 2) Canonical query families

Implemented baseline families:
- `workspace`: workspace summary card over live refs.
- `governance`: table over governance object/lifecycle/apply refs.
- `events`: timeline over event/decision/evidence refs.
- `evidence`: detail record over decision/evidence authority context.
- `authority`: table over authority state + resolution refs.
- `artifacts`: table over artifact metadata + linkage refs.
- `graph`: summary card over graph truth vs transient cognition refs.

## 3) Runtime contracts (canonical commands)

Runtime command IDs:
- `yai.workspace.query` (generic; family from argv[0])
- `yai.workspace.governance.list`
- `yai.workspace.events.tail`
- `yai.workspace.evidence.list`
- `yai.workspace.authority.list`
- `yai.workspace.artifacts.list`
- `yai.workspace.graph.summary`

All return:
- `type: yai.workspace.query.result.v1`
- `query_family`
- `result_shape`
- `workspace_id`
- typed refs linked to canonical sinks.

## 4) Result shapes

Baseline shapes used in DP-8:
- `summary_card`
- `table`
- `detail_record`
- `timeline`

This is the SDK/CLI contract anchor for rendering tables/details without backend leakage.

## 5) Non-leakage rules

1. CLI/SDK must call runtime surfaces; no direct backend query in canonical path.
2. Runtime returns typed refs and summaries, not storage internals as API contract.
3. Backend paths may appear only as internal diagnostics/store refs.
4. Law semantics stay normative; runtime remains query mediation owner.

## 6) Pre-pilot operator pack

Minimum pack exposed by DP-8:
- active governance per workspace
- latest event/evidence trail
- authority/artifact state refs
- graph summary (truth vs heat)
- generic workspace data summary

## 7) Verification

Validation scenario:
- `tests/integration/workspace_lifecycle/workspace_operator_query_surfaces_dp8_v1.sh`

Matrix integration:
- `tests/integration/workspace_lifecycle/workspace_final_demo_matrix_v1.sh`

Acceptance for DP-8 baseline:
- all query families return deterministic `yai.workspace.query.result.v1`
- unsupported families fail with `BAD_ARGS` + `unsupported_query_family`
- operator matrix remains green with DP-8 enabled.

## 8) Cross-repo handoff

`yai` now exposes stable query-family contracts.

`sdk` and `cli` should map these contracts to:
- typed request/response bindings,
- table/timeline/detail render helpers,
- operator command taxonomy without backend coupling.

This preserves runtime as canonical data surface authority.
