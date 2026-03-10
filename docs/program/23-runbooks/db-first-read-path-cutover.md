---
id: RB-DATA-PLANE-DB-FIRST-READ-CUTOVER
status: draft
owner: runtime
effective_date: 2026-03-10
revision: 1
depends_on:
  - docs/program/23-runbooks/enforcement-to-record-persistence.md
  - docs/program/23-runbooks/graph-materialization-from-runtime-records.md
  - docs/program/23-runbooks/data-surfaces-and-operator-query-model.md
---

# DB-First Read Path Cutover

## 1) Scope
This runbook closes DP-12 by switching operational read surfaces to Data Plane records as primary source.

## 2) Canonical surfaces in cutover
- `yai.workspace.inspect`
- `yai.workspace.policy_effective`
- `yai.workspace.debug_resolution`
- `yai.workspace.query` families: `workspace`, `governance`, `events`, `evidence`, `enforcement`, `authority`, `artifacts`, `graph`

## 3) Canonical source mapping
- Workspace inspect/effective/debug: runtime state + persisted sink indexes (event/evidence, governance, authority/artifact, enforcement, graph).
- Event/evidence views: event/evidence stores.
- Governance views: governance object/lifecycle/attachment stores.
- Authority/artifact views: authority and artifact metadata stores.
- Graph summary views: persistent graph store (never transient as truth).

## 4) Declassed read sources
The following are no longer primary read sources:
- ad-hoc runtime-only summaries
- file dump views used as canonical status
- path-derived implicit state

Filesystem remains export/debug/archive surface only.

## 5) DB-first read contract
Every cutover surface exposes:
- `read_path.mode = "db_first"`
- `read_path.primary_source`
- `read_path.db_first_ready`
- `read_path.fallback_active`
- `read_path.fallback_reason`
- `read_path.filesystem_primary = false`

## 6) Consistency rules
- If persisted refs are present, surface reads from persisted records first.
- If persisted set is incomplete, surface remains explicit (`fallback_active=true`) instead of silent merge.
- No hidden fallback to file-first truth.

## 7) Fallback/degraded model
Fallback is allowed only as transitional runtime behavior and must be visible in payload via `read_path`.

## 8) Runtime implementation anchor
- `lib/core/session/utils/session_utils_helpers_domains.inc.c`
  - `yai_workspace_db_first_read_model(...)`
- `lib/core/session/utils/session_utils_surface_core.inc.c`
  - DB-first inspect `read_path`
- `lib/core/session/utils/session_utils_surface_views.inc.c`
  - DB-first policy/debug/query `read_path`

## 9) Verification matrix (DP-12)
- inspect DB-first metadata present and coherent.
- policy effective DB-first metadata present and coherent.
- debug resolution DB-first metadata present and coherent.
- query families return DB-first metadata.
- forced partial materialization yields explicit fallback (`enforcement_incomplete`).

## 10) Handoff
This cutover directly enables:
- DP-13 filesystem governance-state decommission.
- DP-14 filesystem hardcoded compliance/archive cleanup.
- DP-16 richer graph read summaries on stable DB-first posture.
