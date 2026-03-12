---
id: RB-DP-AUTHORITY-ARTIFACT-PERSISTENCE
status: draft
owner: runtime
effective_date: 2026-03-10
revision: 1
depends_on:
  - RB-DATA-PLANE
  - RB-DP-STORAGE-CLASSES
  - RB-DP-STORAGE-TOPOLOGY
  - RB-DP-EVENT-EVIDENCE
  - RB-DP-GOV-COMPLIANCE-PERSISTENCE
---

# Authority and Artifact Metadata Store Integration (DP-6)

## 1) Purpose
Persist authority state and artifact metadata as first-class runtime-owned Data Plane domains.

This phase removes dependence on implicit output-only authority summaries and path-only artifact references.

## 2) Canonical domain split

### Authority domain
Persisted records:
- `yai.authority_state.v1`
- `yai.authority_resolution_record.v1`
- `yai.authority.index.v1`

Workspace-scoped sink paths:
- `~/.yai/run/<ws>/authority/authority-state.v1.ndjson`
- `~/.yai/run/<ws>/authority/resolution-state.v1.ndjson`
- `~/.yai/run/<ws>/authority/index.v1.json`

### Artifact metadata domain
Persisted records:
- `yai.artifact_metadata.v1`
- `yai.artifact_governance_linkage.v1`
- `yai.artifact.metadata.index.v1`

Workspace-scoped sink paths:
- `~/.yai/run/<ws>/artifacts/metadata.v1.ndjson`
- `~/.yai/run/<ws>/artifacts/linkage.v1.ndjson`
- `~/.yai/run/<ws>/artifacts/metadata.index.v1.json`

## 3) Store placement in DP topology
Primary placement:
- Authority state in `S-1 Runtime State Store`
- Artifact metadata in `S-6 Artifact Metadata Store`

Cross-links:
- `S-2/S-3` governance state and lifecycle refs
- `S-4/S-5` event/decision/evidence refs

## 4) Runtime write paths
Writers are runtime-owned and executed inside `yai` resolution/apply flows:
- `yai_session_record_resolution_snapshot(...)`
- `yai_workspace_append_authority_artifact_persistence(...)`

No-bypass rules:
- CLI/SDK never write authority/artifact sinks directly.
- Consumer layers cannot treat output text as authority truth.
- Artifact truth cannot be inferred only from filesystem path naming.

## 5) Typed relationship model
Persisted refs cover:
- authority ↔ workspace
- authority ↔ governance object refs
- authority resolution ↔ decision/evidence/event refs
- artifact ↔ workspace
- artifact ↔ governance refs
- artifact ↔ authority/decision/evidence/event refs

## 6) Surface alignment
Workspace JSON surfaces now expose `authority_artifact_persistence` on:
- `yai.workspace.inspect`
- `yai.workspace.policy_effective`
- `yai.workspace.debug_resolution`

Surface block fields:
- `last_authority_ref`
- `last_authority_resolution_ref`
- `last_artifact_ref`
- `last_artifact_linkage_ref`
- `authority_store_ref`
- `artifact_store_ref`

## 7) Filesystem downgrade policy
Filesystem remains valid as export/debug/publication surface.
Primary operational truth for authority and artifact metadata is in the canonical sink records above.

## 8) Verification baseline
DP-6 verification must prove:
1. authority and artifact records are appended on governed action,
2. index files hold stable latest refs,
3. refs link to decision/evidence/event records,
4. inspect/effective/debug return persisted refs,
5. workspace scoping is preserved.

## 9) Limits kept open
Not in DP-6:
- full operator query UX,
- full graph joins over artifact lineage,
- distributed/federated store design.

## 10) Handoff
DP-6 unlocks:
- DP-7 graph truth/transient cognition integration,
- DP-8 operator query surfaces over governance+authority+artifact state,
- stronger pre-pilot evidence trails per workspace.
