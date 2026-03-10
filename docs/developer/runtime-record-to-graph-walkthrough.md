# Runtime Record to Graph Walkthrough (DP-11)

This walkthrough shows graph truth materialization from runtime-persisted records.

## Scenario
- Workspace: digital / remote-publication
- Action: `digital.publish`
- Outcome: governed (`review_required` or `deny`)

## Flow
1. Runtime persists event/decision/evidence + linkage refs (DP-10).
2. Brain storage bridge materializes graph nodes/edges from those refs.
3. Graph index records materialization metadata (`runtime_record_driven`).
4. Transient cognition is updated separately and remains non-authoritative.

## What is persisted

Graph truth (BR-3):
- `~/.yai/run/<ws>/brain/graph/persistent-nodes.v1.ndjson`
- `~/.yai/run/<ws>/brain/graph/persistent-edges.v1.ndjson`
- `~/.yai/run/<ws>/brain/graph/index.v1.json`

Transient cognition (BR-4):
- `~/.yai/run/<ws>/brain/transient/activation-state.v1.ndjson`
- `~/.yai/run/<ws>/brain/transient/working-set.v1.ndjson`

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
- `tests/integration/workspace_lifecycle/workspace_graph_materialization_hooks_dp11_v1.sh`

The smoke validates:
- node/edge persistence from runtime refs,
- provenance fields and index markers,
- no write-on-read regressions for inspect surfaces,
- graph truth vs transient separation.
