---
id: RB-DP-STORAGE-TOPOLOGY
status: draft
owner: runtime
effective_date: 2026-03-09
revision: 1
depends_on:
  - RB-DATA-PLANE
  - RB-DP-STORAGE-CLASSES
---

# Canonical Storage Topology and Persistence Layout (DP-3)

## 1) Purpose
Define the canonical storage topology for the single-node governed Data Plane.

DP-3 answers: where each storage class lives, how stores are separated, how
layout is versioned and validated, and how to avoid both one-store and store-zoo
anti-patterns.

This runbook does not implement all sinks yet. It provides the topology contract
that DP-4..DP-7 implement.

## 2) Logical persistence domains

### DPD-1 Runtime Operational Domain
Contains:
- workspace operational state,
- authority state,
- operational attachment refs,
- runtime-owned session-adjacent governed state.

### DPD-2 Governance and Compliance Domain
Contains:
- parsed sources,
- normalized IR,
- candidate/approved governance objects,
- governance metadata,
- lifecycle-linked governance references.

### DPD-3 Event and Evidence Domain
Contains:
- runtime and governance lifecycle events,
- decision/effect summaries,
- evidence references,
- audit lineage and trace-oriented history.

### DPD-4 Artifact Metadata Domain
Contains:
- artifact references,
- ownership/scope linkage,
- structural metadata,
- governance↔artifact linkage metadata.

### DPD-5 Brain Graph Domain
Contains:
- persistent graph truth,
- semantic/episodic graph relations,
- authority/evidence/domain linked graph relations.

### DPD-6 Transient Cognition Domain
Contains:
- STM,
- activation state,
- hot graph neighborhoods,
- volatile cognition working sets.

## 3) Canonical logical stores

### S-1 Runtime State Store
Primary for DPD-1 operational truth.

### S-2 Governance State Store
Primary for DPD-2 governance/compliance object state.

### S-3 Review and Apply Store
Primary for review/approval/apply lifecycle transitions and history.

Note: S-2 and S-3 may share an initial physical backend, but remain logically
separate stores with explicit boundaries.

### S-4 Event Store
Primary append-oriented operational event store.

### S-5 Evidence and Resolution Store
Primary decision/evidence/resolution history store.

### S-6 Artifact Metadata Store
Primary artifact metadata and linkage store.

### S-7 Brain Graph Store
Primary persistent graph truth store.

### S-8 Transient Cognition Store
Primary volatile cognition/STM store (non-authoritative).

## 4) Canonical store boundaries

### S-1 Runtime State Store
- Allowed: workspace/authority/runtime operational truth.
- Not allowed: full event history, graph truth, transient cognition truth.

### S-2 Governance State Store
- Allowed: governance object materials and canonical object metadata.
- Not allowed: session state, transient cache payload, debug dump truth.

### S-3 Review and Apply Store
- Allowed: lifecycle transitions, eligibility/apply transitions and history.
- Not allowed: generic event stream payload as primary.

### S-4 Event Store
- Allowed: append-only events.
- Not allowed: authoritative mutable runtime/authority truth.

### S-5 Evidence and Resolution Store
- Allowed: decision/evidence/queryable resolution history.
- Not allowed: replacing authoritative runtime state.

### S-6 Artifact Metadata Store
- Allowed: artifact references and ownership/linkage metadata.
- Not allowed: artifact content truth if content belongs to export surfaces.

### S-7 Brain Graph Store
- Allowed: graph truth.
- Not allowed: volatile activation as sole source of truth.

### S-8 Transient Cognition Store
- Allowed: volatile/hot cognition state.
- Not allowed: authority/governance/event/evidence primary truth.

## 5) Physical layout strategy (no premature overfitting)

DP-3 defines store-family layout intent:

```text
data-plane/
  runtime/
  governance/
  review/
  events/
  evidence/
  artifacts/
  graph/
  transient/
  manifests/
  exports/
```

This is a canonical structure model, not a frozen file-path contract.

Rules:
- physical organization must mirror logical boundaries,
- layout must support versioning and validation,
- layout details must not leak into CLI/SDK public contracts.

## 6) Manifest and versioning model

Topology is manifest-managed and versioned.

### Required model elements
- `data-plane.manifest` (global layout + version + compatibility marker),
- store manifests per family (`runtime`, `governance`, `review`, `events`, `evidence`, `artifacts`, `graph`, `transient`),
- migration markers and compatibility flags.

### Validation baseline
- manifest integrity checks,
- layout presence checks,
- version compatibility checks,
- store-boundary consistency checks.

## 7) Single-node locality and workspace-aware partition discipline

DP-3 baseline is:
- single-node,
- local-first,
- workspace-aware,
- no per-workspace-db explosion by default.

Rule:
Workspace is a scope dimension, not necessarily a dedicated physical backend per workspace.

Implication:
- partition/keys/refs carry `workspace_id` where needed,
- stores remain canonical and manageable.

## 8) Filesystem downgrade strategy

Filesystem remains valid as:
- export surface,
- debug surface,
- artifact publication surface,
- fixture/seed/import-export bridge.

Filesystem is not primary truth for:
- runtime operational truth,
- governance lifecycle truth,
- apply state truth,
- canonical event/evidence persistence,
- graph truth.

## 9) Cross-store references and identity model

Cross-store linking must use typed stable IDs, not implicit path coupling.

### Typed reference classes
- workspace ref,
- governance object ref,
- review/apply ref,
- event ref,
- decision/evidence ref,
- artifact ref,
- graph node/edge ref.

Constraint:
No canonical linkage through accidental filesystem naming.

## 10) Relation to law/sdk/cli without layout leakage

### `law`
Provides norms/schemas/lifecycle semantics; does not own low-level physical DP layout.

### `sdk`
Provides contract surfaces into runtime; must not hardcode physical storage topology.

### `cli`
Uses `sdk -> yai`; must not query stores directly.

## 11) Handoff to sink hardening (DP-4..DP-7)

DP-3 unlocks direct implementation of:
- DP-4 Event and Evidence Sink Hardening,
- DP-5 Governance and Compliance Persistence Integration,
- DP-6 Authority and Artifact Metadata Store Integration,
- DP-7 Brain Graph Sink and Transient Cognition Backend.

Because after DP-3 each sink has:
- explicit logical store,
- explicit boundary,
- explicit topological placement,
- explicit non-bypass constraints.

## 12) Risks and open points (intentional)

Still open by design:
- full physical schema details for each store,
- definitive graph backend contract,
- federation/HA/replication,
- complete operator query model,
- final migration choreography for all file-first residues.

DP-3 closes topology ambiguity, not full sink implementation.
