---
id: DATA-LIFECYCLE-RETENTION-MATRIX-v0.1.0
status: draft
owner: runtime
updated: 2026-03-10
related:
  - docs/program/23-runbooks/data-lifecycle-retention-and-tiering.md
---

# Data Lifecycle / Retention Matrix (v0.1.0)

## Tier and retention model by class

| data_class | hot_role | warm_role | cold_role | lineage_preserved | compactable | archive_eligible | scope_key |
|---|---|---|---|---|---|---|---|
| Runtime Operational State | current runtime truth | bounded historical snapshots | optional archived snapshots | yes | partial | yes | workspace_id |
| Governance State | active objects and attach/apply | superseded/inactive lifecycle history | historical governance snapshots | yes (mandatory) | partial | yes | workspace_id + governance_object_id |
| Event Records | recent operational timeline | compacted historical timeline | long-tail archived windows | yes | yes | yes | workspace_id + time_window |
| Decision Records | recent decision surface | summarized history | archive with lineage anchors | yes (mandatory) | partial | yes | workspace_id + decision_id |
| Evidence Records | active obligations/evidence | summarized/compacted evidence history | archive by profile | yes (mandatory) | partial | yes | workspace_id + decision_ref |
| Authority + Artifact Metadata | current authority/artifact truth | linkage history summary | archive history snapshots | yes | partial | yes | workspace_id + authority/artifact refs |
| Graph Truth | canonical core relations | compacted tail summaries | archived graph tail slices | yes (mandatory) | tail-only | yes | workspace_id + node/edge scope |
| Transient Cognition | hot STM/activation | n/a | n/a | no | yes (TTL expiry) | no | workspace_id + session_id |

## Verification cases (model-level, DP-15A)

| case_id | case | expected_result |
|---|---|---|
| LRM-01 | Event record exits hot window | warm/cold eligibility defined; lineage anchor retained |
| LRM-02 | Governance supersede transition | active/superseded tier transition retains chain refs |
| LRM-03 | Evidence detail compaction | obligation lineage and decision linkage preserved |
| LRM-04 | Graph noisy tail growth | compact/prune eligibility without core-node loss |
| LRM-05 | Transient cognition retention | TTL expiry allowed; no promotion to authoritative truth |
| LRM-06 | Workspace scoping under archive | no cross-workspace merge/leakage in rollup metadata |

## DP-15B execution prerequisites
- job orchestration model per class/tier transition
- rollback/degraded semantics for partial compaction
- orphan detection over decision/evidence/graph link chains
- stress profile: volume/churn/isolation verification
