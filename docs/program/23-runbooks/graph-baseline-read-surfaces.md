# DP-16 — Graph Baseline Read Surfaces

This runbook defines the baseline runtime-mediated read surfaces for graph truth.

## Runtime command family
- `yai.workspace.graph.summary`
- `yai.workspace.graph.workspace`
- `yai.workspace.graph.governance`
- `yai.workspace.graph.decision`
- `yai.workspace.graph.artifact`
- `yai.workspace.graph.authority`
- `yai.workspace.graph.evidence`
- `yai.workspace.graph.lineage`
- `yai.workspace.graph.recent`
- Generic entrypoint: `yai.workspace.query <family>` with `graph.*` families.

## Source model
- Primary source: persistent graph truth (`brain/graph/index.v1.json`, nodes/edges NDJSON).
- Runtime still returns DB-first read metadata (`mode=db_first`, `filesystem_primary=false`).
- Transient cognition remains non-authoritative and is exposed only as support metadata in `graph.summary`.

## Result shapes
- `summary_card`: graph, governance, artifact, authority, evidence.
- `graph_neighborhood_table`: workspace graph neighborhood.
- `detail_record`: decision-centered graph view.
- `timeline`: lineage.
- `table`: recent node/edge classes.

## Boundaries
- No direct backend query contract from clients.
- Scope is workspace-bounded by active binding.
- Surface is baseline/read-only and grounded on persisted records.
