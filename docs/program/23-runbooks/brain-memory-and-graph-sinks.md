---
id: RB-BRAIN-MEMORY-GRAPH-SINKS
status: draft
owner: runtime
effective_date: 2026-03-10
revision: 2
depends_on:
  - RB-DATA-PLANE
  - RB-DP-STORAGE-CLASSES
  - RB-DP-STORAGE-TOPOLOGY
  - RB-DP-EVENT-EVIDENCE
  - RB-DP-GOV-COMPLIANCE-PERSISTENCE
  - RB-DP-AUTHORITY-ARTIFACT-PERSISTENCE
---

# Brain Graph Sink and Transient Cognition Backend (DP-7)

## 1) Purpose
Integrate Brain as a structural Data Plane producer/consumer with explicit separation between:
- persistent graph truth (`S-7`),
- transient cognition/STM (`S-8`).

## 2) Canonical split

### Graph truth (persistent)
- semantic and episodic node materialization,
- relation edges,
- graph lineage refs to event/decision/evidence/governance,
- persisted graph summaries and backend stats.

### Graph heat / transient cognition (non-authoritative)
- activation state,
- hot neighborhoods,
- short-term working sets,
- workspace/session-scoped volatile cognition.

Rule: graph heat never replaces graph sovereignty.

## 3) Implemented sink paths (workspace-scoped)
Graph domain (`S-7`):
- `~/.yai/run/<ws>/brain/graph/persistent-nodes.v1.ndjson`
- `~/.yai/run/<ws>/brain/graph/persistent-edges.v1.ndjson`
- `~/.yai/run/<ws>/brain/graph/index.v1.json`

Transient domain (`S-8`):
- `~/.yai/run/<ws>/brain/transient/activation-state.v1.ndjson`
- `~/.yai/run/<ws>/brain/transient/working-set.v1.ndjson`
- `~/.yai/run/<ws>/brain/transient/index.v1.json`

## 4) Runtime write model
Canonical writer hook:
- `yai_mind_storage_bridge_resolution_hook(...)`

Write trigger:
- runtime governed resolution snapshot path (`core/session`) invokes brain hook after event/evidence and authority/artifact persistence.

Degradation rule:
- brain sink failure is non-fatal for runtime correctness (governance/authority correctness must remain intact).

## 5) Hook linkage model
Current hook materializes typed links from governed execution to brain stores:
- decision -> episodic anchor node,
- family/specialization -> semantic anchor node,
- semantic->episodic edge,
- refs to event/decision/evidence/governance context,
- transient activation + transient working-set state.

## 6) Surface alignment
Workspace surfaces expose `brain_persistence` on:
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

## 7) Backend role clarification
- Persistent graph sink is `BR-3`.
- Transient cognition sink is `BR-4`.
- Redis remains a valid candidate for `BR-4` only.

Explicit boundary:
- Redis for graph heat, not for graph sovereignty.

## 8) No-bypass and authority rules
1. Transient cognition is explicitly non-authoritative.
2. Graph truth mutations must pass canonical runtime writer paths.
3. CLI/SDK remain consumers through runtime contracts, never direct backend writers.
4. Governance/authority truth cannot migrate into transient memory.

## 9) Verification baseline
DP-7 smoke must verify:
1. graph and transient sinks are both written,
2. graph index reports `graph_truth_authoritative=true`,
3. transient index reports `authoritative=false`,
4. workspace inspect/effective/debug expose brain refs,
5. refs are decision/evidence-linked and workspace-scoped.

## 10) Handoff
DP-7 unlocks:
- DP-8 operator query surfaces over graph/evidence/governance/authority,
- DP-9 qualification closure with graph-vs-transient guardrails proven.
