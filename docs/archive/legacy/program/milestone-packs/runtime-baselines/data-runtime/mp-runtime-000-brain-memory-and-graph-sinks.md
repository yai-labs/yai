---
id: RB-BRAIN-MEMORY-GRAPH-SINKS
status: active
owner: runtime
effective_date: 2026-03-10
revision: 3
depends_on:
  - RB-DATA-PLANE
  - RB-DP-STORAGE-CLASSES
  - RB-DP-STORAGE-TOPOLOGY
  - RB-DP-EVENT-EVIDENCE
  - RB-DP-GOV-COMPLIANCE-PERSISTENCE
  - RB-DP-AUTHORITY-ARTIFACT-PERSISTENCE
---

# Graph Sink and Transient Knowledge Backend (DP-7)

Legacy filename kept for trace continuity. Canonical model is unified runtime
families with workspace-first binding.

## 1) Purpose

Integrate graph and transient knowledge-state sinks as first-class data-plane
producers/consumers with explicit separation between:
- persistent graph truth (`graph` family),
- transient knowledge-state heat (`knowledge` support).

## 2) Canonical split

### Graph truth (persistent)

- semantic and episodic node materialization,
- relation edges,
- graph lineage refs to event/decision/evidence/governance,
- persisted graph summaries and backend stats.

### Transient knowledge state (non-authoritative)

- activation state,
- hot neighborhoods,
- short-term working sets,
- workspace/session-scoped volatile context.

Rule: transient state never replaces authoritative graph truth.

## 3) Implemented sink paths (workspace-scoped)

Graph domain:
- `~/.yai/run/<ws>/runtime/graph/persistent-nodes.v1.ndjson`
- `~/.yai/run/<ws>/runtime/graph/persistent-edges.v1.ndjson`
- `~/.yai/run/<ws>/runtime/graph/index.v1.json`

Transient domain:
- `~/.yai/run/<ws>/runtime/transient/activation-state.v1.ndjson`
- `~/.yai/run/<ws>/runtime/transient/working-set.v1.ndjson`
- `~/.yai/run/<ws>/runtime/transient/index.v1.json`

## 4) Runtime write model

Canonical writer path:
- runtime resolution snapshot flow emits persisted records first,
- graph/transient materialization hooks run after event/evidence/authority
  persistence.

Degradation rule:
- transient sink failure is non-fatal for governance correctness.
- graph persistence degradation must be surfaced as degraded runtime readiness.

## 5) Surface alignment

Workspace surfaces expose persistence refs on:
- `yai.workspace.inspect`
- `yai.workspace.policy_effective`
- `yai.workspace.debug_resolution`

Fields:
- `last_graph_node_ref`
- `last_graph_edge_ref`
- `last_transient_state_ref`
- `last_transient_working_set_ref`
- `graph_store_ref`
- `transient_store_ref`
- `graph_truth_authoritative`
- `transient_authoritative`

## 6) No-bypass and authority rules

1. Transient knowledge state is explicitly non-authoritative.
2. Graph truth mutations pass canonical runtime writer paths.
3. CLI/SDK are consumers through runtime contracts, never direct backend writers.
4. Governance/authority truth cannot move into transient memory.

## 7) Verification baseline

DP-7 smoke verifies:
1. graph and transient sinks are both written,
2. graph index reports `graph_truth_authoritative=true`,
3. transient index reports `authoritative=false`,
4. workspace inspect/effective/debug expose refs,
5. refs are decision/evidence-linked and workspace-scoped.

## 8) Handoff

DP-7 unlocks:
- DP-8 operator query surfaces over graph/evidence/governance/authority,
- DP-9 qualification closure with graph-vs-transient guardrails proven.
