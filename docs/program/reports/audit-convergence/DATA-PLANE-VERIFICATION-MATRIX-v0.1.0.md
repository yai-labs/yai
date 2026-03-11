---
id: DATA-PLANE-VERIFICATION-MATRIX-v0.1.0
status: draft
owner: runtime
updated: 2026-03-10
scope: DP-1..DP-9 closure baseline
---

# Data Plane Verification Matrix (v0.1.0)

Legend:
- Status: `confirmed | partial | not_present`
- Evidence: primary script/doc anchor proving the status.

## A) Model integrity

| Check | Status | Evidence | Notes / Residual gap |
|---|---|---|---|
| Storage classes defined | confirmed | `docs/program/milestone-packs/runtime-baselines/data-plane-storage-classes.md` | Baseline complete. |
| Backend role model defined | confirmed | `docs/program/milestone-packs/runtime-baselines/data-plane-storage-classes.md` | Role fit defined; backend final choices still selective. |
| Canonical topology defined | confirmed | `docs/program/milestone-packs/runtime-baselines/data-plane-storage-topology.md` | Single-node topology baseline. |
| Non-bypass rules declared | confirmed | `docs/program/milestone-packs/runtime-baselines/data-plane.md` | Enforced through runtime contract path. |

## B) Persistence integrity

| Check | Status | Evidence | Notes / Residual gap |
|---|---|---|---|
| Event sink persisted | confirmed | `workspace_event_evidence_sink_hardening.sh` | Append baseline active. |
| Evidence/decision sink persisted | confirmed | `workspace_event_evidence_sink_hardening.sh` | Typed records + refs. |
| Governance persistence active | confirmed | `workspace_governance_persistence.sh` | Candidate/approved/apply refs baseline. |
| Authority persistence active | confirmed | `workspace_authority_artifact_persistence.sh` | Runtime-owned refs and index. |
| Artifact metadata persistence active | confirmed | `workspace_authority_artifact_persistence.sh` | Linkage refs persisted. |
| Brain graph truth persisted | confirmed | `workspace_brain_graph_transient.sh` | BR-3 role modeled. |
| Transient cognition separated | confirmed | `workspace_brain_graph_transient.sh` | Explicit non-authoritative. |

## C) Runtime integration

| Check | Status | Evidence | Notes / Residual gap |
|---|---|---|---|
| Runtime writer paths canonical | confirmed | `lib/core/session/session_utils.c` + matrix scripts | Core writes sinks; no CLI direct writes. |
| Inspect/effective/debug aligned to persisted refs | confirmed | `workspace_inspect_surfaces.sh` | Multi-domain refs visible. |
| Workspace↔governance linkage visible | confirmed | `workspace_governance_apply_semantics.sh` | Attachment/apply state surfaced. |
| Governance lifecycle gating enforced | confirmed | `workspace_review_approval_gate.sh` | Candidate blocked without approval path. |

## D) Surface integrity

| Check | Status | Evidence | Notes / Residual gap |
|---|---|---|---|
| Operator query families exposed | confirmed | `workspace_operator_query_surfaces.sh` | governance/events/evidence/authority/artifacts/graph/workspace. |
| Deterministic result shapes | confirmed | `workspace_operator_query_surfaces.sh` | table/timeline/detail_record/summary_card. |
| No backend leakage in canonical contract | confirmed | `docs/program/milestone-packs/runtime-baselines/data-surfaces-and-operator-query-model.md` | Backend roles remain internal detail. |
| CLI matrix readability preserved | confirmed | `workspace_demo_matrix.sh` | End-to-end scenario remains green. |

## E) Safety and discipline

| Check | Status | Evidence | Notes / Residual gap |
|---|---|---|---|
| Lifecycle no-skip respected | confirmed | `workspace_agent_safe_boundaries.sh` | review/approval boundaries enforced. |
| Workspace scoping respected | confirmed | `workspace_session_binding_contract.sh` + `workspace_negative_paths.sh` | stale/invalid handled explicitly. |
| Transient is non-authoritative | confirmed | `workspace_brain_graph_transient.sh` | Declared and validated in index/state. |
| Consumer bypass blocked as canonical flow | confirmed | runtime command mediation in `session.c` | Contract path remains runtime-mediated. |

## F) Block qualification outcome

- `QC-1..QC-5`: **confirmed** for v0.1.0 single-node baseline.
- Ready for pre-pilot technical slice with explicit residual risks.
