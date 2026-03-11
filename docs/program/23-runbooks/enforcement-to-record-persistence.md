---
id: RB-DP-10-ENFORCEMENT-RECORD-CLOSURE
title: Enforcement-to-Record Persistence Closure
status: draft
owner: runtime
effective_date: 2026-03-10
depends_on:
  - RB-EVENT-EVIDENCE-PERSISTENCE
  - RB-GOVERNANCE-COMPLIANCE-PERSISTENCE
  - RB-AUTHORITY-ARTIFACT-PERSISTENCE
  - RB-BRAIN-MEMORY-GRAPH-SINKS
  - RB-DATA-SURFACES
tags:
  - data-plane
  - enforcement
  - persistence
  - runtime
---

# DP-10 — Enforcement-to-Record Persistence Closure

## 1) Purpose
Close the canonical writer path `enforcement -> persisted record set`.

Runtime outcomes are valid only when materialized as persistent typed records,
not only printed/debugged.

## 2) Outcomes covered
Baseline closure applies to governed runtime outcomes:
- `allow`
- `review_required`
- `deny`
- `quarantine` (when emitted by policy/effect)
- apply/review gate outcomes emitted through runtime flow.

## 3) Canonical record set
For each relevant outcome, runtime now writes:
1. runtime event record
2. decision record
3. evidence record
4. governance linkage refs (when present)
5. authority linkage refs (when present)
6. artifact linkage refs (when present)
7. enforcement outcome/linkage record pair

## 4) DP-10 sink contract
Workspace sink family:
- `~/.yai/run/<ws>/enforcement/outcome-records.v1.ndjson`
- `~/.yai/run/<ws>/enforcement/linkage-records.v1.ndjson`
- `~/.yai/run/<ws>/enforcement/index.v1.json`

Index type:
- `yai.enforcement.recordset.index.v1`

NDJSON types:
- `yai.enforcement_outcome_record.v1`
- `yai.enforcement_linkage_record.v1`

## 5) Completeness and failure semantics
Index includes completeness status:
- `materialization_status: complete|incomplete`
- `missing_fields`

This avoids silent partial persistence.
If a write set is partial, runtime exposes explicit incomplete state via:
- sink index
- policy effective/debug/inspect/query surfaces.

## 6) Runtime writers and orchestration
Writer orchestration is runtime-owned (`yai`), not CLI/SDK-owned.

Key runtime/mapping anchors:
- `lib/core/session/session_utils.c`
- `lib/law/mapping/decision_to_evidence.c`
- `lib/law/mapping/decision_to_audit.c`

Canonical order:
1. enforce outcome
2. emit event/decision/evidence
3. emit governance/authority/artifact linkage
4. emit enforcement outcome/linkage records
5. publish refs on runtime surfaces.

## 7) Surface alignment
Runtime surfaces now expose `enforcement_record_set`:
- `yai.workspace.inspect`
- `yai.workspace.policy_effective`
- `yai.workspace.debug_resolution`
- `yai.workspace.query enforcement`

This keeps the read path runtime-mediated (`cli -> sdk -> yai`) and
prepares DB-first reader cutover.

## 8) Verification
Smoke/integration:
- `tests/integration/workspace/workspace_enforcement_record_closure.sh`

Coverage:
- complete materialization baseline
- forced partial materialization baseline
- inspect/query visibility of completeness status.

Matrix inclusion:
- `tests/integration/workspace/workspace_demo_matrix.sh`

## 9) Handoff
DP-10 enables:
- DP-11 graph materialization from typed persisted refs
- DP-12 DB-first read path cutover from canonical writer outputs

Writer closure first, reader cutover after.
