---
id: RB-FILESYSTEM-CLEANUP-AND-ARCHIVE-EXECUTION
status: draft
owner: runtime
effective_date: 2026-03-10
revision: 1
depends_on:
  - docs/archive/legacy/program/milestone-packs/runtime-baselines/governance-runtime/mp-runtime-000-filesystem-decommission.md
  - docs/archive/legacy/program/milestone-packs/runtime-baselines/data-runtime/mp-runtime-000-db-first-read-path-cutover.md
---

# DP-14 — Hardcoded Compliance/Policy Filesystem Cleanup and Archive

## 1) Scope
This runbook executes DP-13 matrix decisions.

## 2) Archive zone
Operational archive is standardized at:
- `../archive/data-plane-filesystem/`

Rules:
- runtime/SDK/CLI must not read archive paths as primary truth.
- archive is transitional and non-authoritative.

## 3) Executed moves
- `governance/transitional/domain-family-seed/**`
  -> `../archive/data-plane-filesystem/governance/transitional/domain-family-seed/**`

Tracked in:
- `../archive/data-plane-filesystem/notes/RELOCATION_MAP.md`
- `docs/archive/legacy/program/reports/filesystem-governance-convergence-report.md`

## 4) Operational cleanup outcome
- Transitional seed residue removed from `governance` operational path.
- Normative sources were preserved (`governance/grammar`, `governance/domains`, `governance/overlays`, canonical manifests).
- Export/embedded surfaces kept with non-primary role.

## 5) Safety checks
Post-cleanup validation:
- `tests/integration/workspace/workspace_db_first_read_cutover.sh`
- `tests/integration/workspace/workspace_demo_matrix.sh`

## 6) Remaining residues (planned DP-15)
Still transitional in runtime path and not yet removable:
- `~/.yai/run/<ws>/workspace-state.json`
- `~/.yai/run/<ws>/runtime-state.json`
- `~/.yai/run/<ws>/binding-state.json`

These require binder/read mediation closure before operational removal.

## 7) Handoff
- DP-15: stronger DB-backed governance/compliance visibility with less filesystem ambiguity.
- DP-16: cleaner graph read/summaries on de-ambiguous runtime filesystem posture.
