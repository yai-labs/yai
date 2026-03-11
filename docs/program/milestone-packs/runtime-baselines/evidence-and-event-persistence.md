---
id: RB-DP-EVENT-EVIDENCE
status: draft
owner: runtime
effective_date: 2026-03-09
revision: 1
depends_on:
  - RB-DATA-PLANE
  - RB-DP-STORAGE-CLASSES
  - RB-DP-STORAGE-TOPOLOGY
---

# Event and Evidence Sink Hardening (DP-4)

## 1) Purpose
DP-4 hardens the first canonical Data Plane writers:
- runtime/governance events,
- decision records,
- evidence records.

Goal: move from file-ish debug fallout to runtime-owned persisted records with
stable IDs and workspace scoping.

## 2) Canonical classes in scope

### Event classes (initial)
- `E-1 runtime_operational`: runtime up/down, bounded runtime transitions.
- `E-2 workspace`: workspace set/unset/domain changes and scoped transitions.
- `E-3 governance_lifecycle`: review/approval/apply-related transitions.
- `E-4 execution_governance`: governed action resolution emitted by runtime.

### Resolution/evidence classes (initial)
- `R-1 decision_record`: effect+rationale+stack/precedence outcome.
- `R-2 evidence_record`: obligations/provenance context for a decision.
- `R-3 resolution_summary`: workspace-visible compact resolution summary.
- `R-4 trace_link`: stable refs across event/decision/evidence/trace.

## 3) Initial sink topology placement
Aligned with DP-3 `DPD-3` domain:
- `~/.yai/run/<ws>/events/runtime-events.v1.ndjson`
- `~/.yai/run/<ws>/events/decision-records.v1.ndjson`
- `~/.yai/run/<ws>/events/evidence-records.v1.ndjson`
- `~/.yai/run/<ws>/events/index.v1.json`

`traces/index.json` remains available as workspace trace surface. It is no
longer the only traceable surface for operational evidence.

## 4) Structured record model (v1 baseline)

### Runtime event record (`yai.runtime_event.v1`)
Minimum fields:
- `event_id`, `event_type`, `event_class`,
- `timestamp`, `timestamp_epoch`,
- `workspace_id`, `source_component`, `actor_class`,
- `target_ref`, `status`,
- `trace_ref`, `decision_ref`, `evidence_ref`,
- payload summary (`family_id`, `specialization_id`, `effect`).

### Decision record (`yai.decision_record.v1`)
Produced by mapper (`decision_to_audit`) with:
- identity (`decision_id`, family/domain/specialization),
- effect/rationale,
- stack and precedence refs,
- overlay counters,
- authority/evidence profile summaries.

### Evidence record (`yai.evidence_record.v1`)
Produced by mapper (`decision_to_evidence`) with:
- `decision_ref`, `trace_ref`, workspace scope marker,
- family/domain/specialization/effect/provider/resource context,
- `authority_context`, `evidence_profile`,
- hardening requirement flags
  (`review_trace_required`, `retention_required`, `provenance_required`,
  `approval_chain_required`, `dependency_chain_required`,
  `lawful_basis_required`, `oversight_trace_required`).

## 5) Canonical write paths
- Runtime writer anchor: `yai_session_record_resolution_snapshot(...)`.
- Mapper hardening points:
  - `lib/law/mapping/decision_to_audit.c`
  - `lib/law/mapping/decision_to_evidence.c`
- CLI/SDK remain readers through `yai` surfaces; no direct sink writes.

## 6) Append baseline semantics
- Append-only NDJSON logs for events/decision/evidence records.
- Workspace-scoped index (`index.v1.json`) captures latest typed refs and store
  locations.
- On sink write failure, runtime returns canonical failure; no silent drop.

## 7) Inspect/debug/effective surface alignment
Workspace surfaces now include `event_evidence_sink` metadata:
- `last_event_ref`, `last_decision_ref`, `last_evidence_ref`,
- `event_store_ref`, `decision_store_ref`, `evidence_store_ref`.

This keeps user-facing inspect deterministic while anchoring to persisted sinks.

## 8) Role-fit guidance
- Primary role for event/evidence sinks: `BR-2 append+tabular-query`.
- Current baseline storage form: workspace-local append logs + index.
- Filesystem role here is transitional sink substrate, not ad hoc debug-only
  output.

## 9) Verification baseline
DP-4 requires smoke/integration coverage for:
- append of all three streams (event/decision/evidence),
- stable refs in index,
- workspace inspect/debug/effective visibility of sink refs.

## 10) Handoff
DP-4 unlocks:
- `DP-5 Governance and Compliance Persistence Integration`
- `DP-8 CLI/SDK Data Surfaces and Operator Query Model`

because runtime now materializes canonical event/evidence records instead of
only ephemeral trace output.
