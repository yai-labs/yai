# Runtime Record to Graph Walkthrough (DP-11)

This walkthrough validates graph truth materialization from runtime-persisted
records in a workspace-bound flow.

## Scenario

- Workspace context: `digital/remote-publication`
- Action: `digital.publish`
- Outcome: governed (`review_required` or `deny`)

## Flow

1. Runtime persists event/decision/evidence + linkage refs (DP-10).
2. Runtime graph materialization hook writes graph nodes/edges from those refs.
3. Graph index records materialization metadata (`runtime_record_driven`).
4. Transient knowledge state is updated separately and remains non-authoritative.

## What is persisted

Graph truth:
- `~/.yai/run/<ws>/runtime/graph/persistent-nodes.v1.ndjson`
- `~/.yai/run/<ws>/runtime/graph/persistent-edges.v1.ndjson`
- `~/.yai/run/<ws>/runtime/graph/index.v1.json`

Transient knowledge state:
- `~/.yai/run/<ws>/runtime/transient/activation-state.v1.ndjson`
- `~/.yai/run/<ws>/runtime/transient/working-set.v1.ndjson`

## Node classes baseline

- workspace_node
- governance_object_node
- decision_node
- evidence_node
- authority_node
- artifact_node
- runtime_episode_node

## Edge classes baseline

- decision_in_workspace
- decision_under_governance
- decision_under_authority
- decision_on_artifact
- evidence_for_decision
- artifact_governed_by
- workspace_uses_governance
- episode_yielded_decision

## Smoke command

- `tests/integration/workspace/workspace_graph_materialization_hooks.sh`

The smoke validates:
- node/edge persistence from runtime refs,
- provenance fields and index markers,
- no write-on-read regressions for inspect surfaces,
- authoritative graph truth vs non-authoritative transient separation.
