---
id: RB-FILESYSTEM-GOVERNANCE-STATE-DECOMMISSION
status: draft
owner: runtime
effective_date: 2026-03-10
revision: 1
depends_on:
  - docs/program/23-runbooks/db-first-read-path-cutover.md
  - docs/program/23-runbooks/governance-and-compliance-persistence.md
  - docs/program/23-runbooks/enforcement-to-record-persistence.md
  - docs/program/23-runbooks/graph-materialization-from-runtime-records.md
---

# DP-13 — Filesystem Governance State Decommission Plan

## 1) Why now
After DP-12, writer and reader posture is DB-first. Remaining risk is filesystem ambiguity: files that still look like runtime truth.

DP-13 is planning-only and produces an execution map for DP-14.

## 2) Scope inventory
Repositories covered:
- `yai` (primary)
- `law` (normative source + generated/export surfaces)
- `sdk` (dist/examples)
- `cli` (dist)

Runtime-sensitive path classes covered:
- governance/compliance manifests and policy packs
- event/evidence/trace/debug outputs
- generated/embedded exports
- transitional seed artifacts

## 3) Canonical classification model
- `FS-C1 Normative Source of Truth`: canonical authoring and schemas.
- `FS-C2 Build/Export Derivative`: generated/embed/export artifacts, not operational truth.
- `FS-C3 Debug/Diagnostic Surface`: trace/debug snapshots, not primary reads.
- `FS-C4 Transitional Residue`: temporary migration scaffolding.
- `FS-C5 False Operational State`: file surfaces that still look/behave like runtime truth.
- `FS-C6 Archive Candidate`: keep history, move out of operational path (`archive_tmp`).

## 4) Decision matrix
Canonical matrix is:
- `docs/program/audit-convergence/FILESYSTEM-GOVERNANCE-DECOMMISSION-MATRIX-v0.1.0.md`

Required fields per row:
- path
- current class / target class
- runtime-read status
- keep/export/debug/archive/remove decision
- migration dependency
- cleanup phase owner

## 5) Governance/compliance hardcoded JSON focus
Classification policy:
- `law/grammar/**`, `law/domains/**`, `law/overlays/**`, `law/manifests/law.manifest.json`: `FS-C1` keep canonical.
- `law/manifests/customer-policy-packs/examples/**`, `templates/**`: `FS-C2` (authoring examples/templates), never runtime truth.
- `law/transitional/domain-family-seed/**`: `FS-C4 -> FS-C6` (archive candidate).
- `yai/embedded/law/**`: `FS-C2` generated embedded export, keep as runtime package export, not governance runtime state truth.

## 6) Evidence/trace/debug residue focus
- `~/.yai/run/<ws>/events/*.ndjson` and domain indexes are canonical persisted records for current single-node runtime.
- trace/debug exports remain `FS-C3` and must not be used as primary read source.
- operator surfaces must stay on DB-first payloads with explicit `read_path` status.

## 7) Embedded/generated split
Keep:
- generated embedded law bundles needed by runtime loader (`FS-C2`).

Declass:
- generated files used as surrogate runtime state (`FS-C5`).

## 8) Operational path removal rules
A file/path is removed from operational path if all are true:
1. equivalent object exists in Data Plane persistent stores,
2. primary reads are DB-first for affected surfaces,
3. file usage is export/debug/transitional only,
4. fallback does not silently depend on that file.

## 9) `archive_tmp` strategy
Target holding area:
- `archive_tmp/data-plane-filesystem/` (per repo where needed).

Rules:
- runtime never reads from `archive_tmp`.
- CLI/SDK never use `archive_tmp` as canonical source.
- every moved item carries `archived/transitional` marker in matrix.

## 10) Repo-by-repo map
- `yai`: decommission false operational residues; keep persisted sink logs/indexes currently required by runtime.
- `law`: preserve normative source; archive transitional seeds and non-canonical residues.
- `sdk`: declassify `dist/examples` as derivative/package artifacts.
- `cli`: declassify `dist` as derivative/package artifact.

## 11) Verification baseline
Verification evidence for "read path no longer depends on decommission targets":
- `tests/integration/workspace_lifecycle/workspace_db_first_read_cutover_dp12_v1.sh`
- `tests/integration/workspace_lifecycle/workspace_final_demo_matrix_v1.sh`

Required payload invariants (already enforced by DP-12):
- `read_path.mode = db_first`
- `read_path.filesystem_primary = false`
- explicit fallback markers when partial/degraded

## 12) Developer walkthrough
- `docs/developer/filesystem-decommission-walkthrough.md`

## 13) Handoff to DP-14
DP-14 executes the matrix mechanically:
- move `FS-C6` targets to `archive_tmp`
- remove `FS-C5` from operational paths
- keep `FS-C1/C2/C3` with explicit role labels

## 14) Handoff to DP-15/DP-16
- DP-15 gains cleaner governance/compliance visibility without file-first ambiguity.
- DP-16 gains cleaner graph summary surfaces over a de-ambiguous filesystem baseline.
