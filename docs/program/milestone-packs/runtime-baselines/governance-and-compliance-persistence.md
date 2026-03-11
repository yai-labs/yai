---
id: RB-DP-GOV-COMPLIANCE-PERSISTENCE
status: draft
owner: runtime
effective_date: 2026-03-10
revision: 1
depends_on:
  - RB-DATA-PLANE
  - RB-DP-STORAGE-CLASSES
  - RB-DP-STORAGE-TOPOLOGY
  - RB-DP-EVENT-EVIDENCE
---

# Governance and Compliance Persistence Integration (DP-5)

## 1) Purpose
Treat governance/compliance custom state as a first-class persisted Data Plane
domain, not only as repo artifacts/manifests.

## 2) Normative source vs live persisted state

### Normative source (`law`)
- schemas/grammar/authoring manifests,
- canonical IDs and validation vocabulary.

### Live persisted state (`yai`)
- runtime-relevant object state,
- lifecycle state,
- workspace attachment/apply linkage,
- stable refs to event/decision/evidence trail.

## 3) Persisted governance classes (v1)
- `GPC-4` candidate/approved governance object state.
- `GPC-6` lifecycle state (review/approval/apply eligibility summary).
- `GPC-7` workspace attachment/apply linkage.
- `GPC-8` lineage refs toward event/evidence records.

Initial persistence paths (workspace-scoped):
- `~/.yai/run/<ws>/governance/object-state.v1.ndjson`
- `~/.yai/run/<ws>/governance/lifecycle-state.v1.ndjson`
- `~/.yai/run/<ws>/governance/attachment-state.v1.ndjson`
- `~/.yai/run/<ws>/governance/index.v1.json`

## 4) Store placement
Primary logical placement:
- `S-2 Governance State Store`
- `S-3 Review and Apply Store`

Cross-links:
- `S-1` runtime context (`workspace_id`, binding state),
- `S-4/S-5` event/decision/evidence refs.

## 5) Runtime write paths
- Attach/activate/detach flows write governance object+lifecycle+attachment rows.
- Governed resolution snapshots write linkage rows (`resolution_link`) for active
  attachments with event/decision/evidence refs.
- CLI remains a consumer through runtime contracts, never direct store writer.

## 6) Relationship model

### Governance ↔ Workspace
Typed attachment record:
- `attachment_id`, `workspace_id`, `governance_object_id`,
- action (`attach`, `activate`, `detach`, `resolution_link`),
- eligibility/compatibility/conflicts/warnings.

### Governance ↔ Event/Evidence
Typed refs in attachment/index records:
- `event_ref`,
- `decision_ref`,
- `evidence_ref`.

## 7) Filesystem downgrade for governance operations
Filesystem remains valid for authoring/export/debug.
Primary operational truth for governance state is now the governance sink records
written by runtime flows.

## 8) Surface alignment
Workspace inspect/effective/debug now expose `governance_persistence`:
- latest object/lifecycle/attachment refs,
- store refs for objects/lifecycle/attachments.

This keeps runtime inspectability and prepares query surfaces in DP-8.

## 9) Verification baseline
DP-5 smoke must demonstrate:
1. governance object persisted after attach,
2. lifecycle record persisted,
3. attachment state persisted,
4. linkage to event/decision/evidence after governed action,
5. workspace surfaces expose governance persistence refs.

## 10) Handoff
DP-5 unlocks:
- DP-6 authority+artifact metadata persistence integration,
- DP-8 operator query surfaces over live governance state.
