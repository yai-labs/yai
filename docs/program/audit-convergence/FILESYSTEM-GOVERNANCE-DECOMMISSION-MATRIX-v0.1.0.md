---
id: DATA-PLANE-FILESYSTEM-DECOMMISSION-MATRIX-v0.1.0
status: draft
owner: runtime
updated: 2026-03-10
related:
  - docs/program/23-runbooks/filesystem-governance-state-decommission.md
  - docs/program/23-runbooks/db-first-read-path-cutover.md
---

# Filesystem Governance State Decommission Matrix (v0.1.0)

## Legend
- Classes: `FS-C1..FS-C6` from DP-13 runbook.
- `runtime_read`: current runtime primary read dependency.
- `docs_tooling_only`: consumed only by docs/build/tooling.

## Decision Matrix

| path | owner_repo | current_class | target_class | runtime_read | docs_tooling_only | remain_canonical | export_only | archive | remove_operational | migration_dependency | cleanup_phase | notes |
|---|---|---:|---:|---|---|---|---|---|---|---|---|---|
| `law/grammar/**` | `law` | FS-C1 | FS-C1 | yes (indirect via embed) | no | yes | no | no | no | none | DP-13 | Normative schema/grammar source. |
| `law/domains/**` | `law` | FS-C1 | FS-C1 | yes (indirect via embed) | no | yes | no | no | no | none | DP-13 | Domain canonical source. |
| `law/overlays/**` | `law` | FS-C1 | FS-C1 | yes (indirect via embed) | no | yes | no | no | no | none | DP-13 | Overlay canonical source. |
| `law/manifests/law.manifest.json` | `law` | FS-C1 | FS-C1 | yes (indirect via embed) | no | yes | no | no | no | none | DP-13 | Canonical law root manifest. |
| `law/manifests/embedded-export.manifest.json` | `law` | FS-C2 | FS-C2 | no | yes | no | yes | no | yes | build/export compatibility | DP-14 | Derivative; not runtime state truth. |
| `law/manifests/customer-policy-packs/examples/**` | `law` | FS-C2 | FS-C2 | no | yes | no | yes | no | yes | none | DP-14 | Example authoring inputs only. |
| `law/manifests/customer-policy-packs/templates/**` | `law` | FS-C2 | FS-C2 | no | yes | no | yes | no | yes | none | DP-14 | Templates only. |
| `law/transitional/domain-family-seed/**` | `law` | FS-C4 | FS-C6 | no | yes | no | no | yes | yes | none | DP-14 (executed 2026-03-10) | Transitional residue moved to archive_tmp. |
| `yai/embedded/law/manifests/**` | `yai` | FS-C2 | FS-C2 | yes | no | no | yes | no | no | runtime loader packaging | DP-14 | Keep as packaged export surface. |
| `yai/embedded/law/generated/**` | `yai` | FS-C2 | FS-C2 | yes | no | no | yes | no | no | runtime loader packaging | DP-14 | Generated embed artifacts; non-authoritative governance state. |
| `~/.yai/run/<ws>/events/*.ndjson` | runtime | FS-C5 | FS-C5 (interim) | yes | no | no | no | no | no (now) | sink backend migration | DP-14+ | Current persisted record store (single-node). |
| `~/.yai/run/<ws>/events/index.v1.json` | runtime | FS-C5 | FS-C5 (interim) | yes | no | no | no | no | no (now) | read mediator migration | DP-14+ | Required by current DB-first mediation. |
| `~/.yai/run/<ws>/governance/*.ndjson` | runtime | FS-C5 | FS-C5 (interim) | yes | no | no | no | no | no (now) | governance backend migration | DP-14+ | Current governance persisted substrate. |
| `~/.yai/run/<ws>/authority/*.ndjson` | runtime | FS-C5 | FS-C5 (interim) | yes | no | no | no | no | no (now) | authority backend migration | DP-14+ | Current authority persisted substrate. |
| `~/.yai/run/<ws>/artifacts/*.ndjson` | runtime | FS-C5 | FS-C5 (interim) | yes | no | no | no | no | no (now) | artifact backend migration | DP-14+ | Current artifact persisted substrate. |
| `~/.yai/run/<ws>/graph/*.ndjson` | runtime | FS-C5 | FS-C5 (interim) | yes | no | no | no | no | no (now) | graph backend migration | DP-14+ | Current graph truth persisted substrate. |
| `~/.yai/run/<ws>/transient/*.ndjson` | runtime | FS-C3 | FS-C3 | yes | no | no | no | no | no | none | DP-14 | Non-authoritative transient/debug storage. |
| `~/.yai/run/<ws>/workspace-state.json` | runtime | FS-C5 | FS-C4 | yes | no | no | no | yes (future) | yes (planned) | binding model sink-native | DP-14/15 | Transitional summary file. |
| `~/.yai/run/<ws>/runtime-state.json` | runtime | FS-C5 | FS-C4 | yes | no | no | no | yes (future) | yes (planned) | runtime state mediation closure | DP-14/15 | Transitional runtime snapshot. |
| `~/.yai/run/<ws>/binding-state.json` | runtime | FS-C5 | FS-C4 | yes | no | no | no | yes (future) | yes (planned) | binder sink closure | DP-14/15 | Transitional binder compatibility file. |
| `sdk/dist/**` | `sdk` | FS-C2 | FS-C2 | no | yes | no | yes | no | yes | package pipeline | DP-14 | Build derivative only. |
| `sdk/examples/**` | `sdk` | FS-C2 | FS-C2 | no | yes | no | yes | no | yes | none | DP-14 | Example usage surface. |
| `cli/dist/**` | `cli` | FS-C2 | FS-C2 | no | yes | no | yes | no | yes | package pipeline | DP-14 | Build derivative only. |

## Archive Plan (`archive_tmp`)
Planned non-operational destination:
- `archive_tmp/data-plane-filesystem/`

Initial archive candidates:
- `law/transitional/domain-family-seed/**`
- additional FS-C4 residues discovered during DP-14 execution sweep.

## Verification Evidence
- DB-first read cutover smoke: `tests/integration/workspace_lifecycle/workspace_db_first_read_cutover_dp12_v1.sh`
- full runtime regression: `tests/integration/workspace_lifecycle/workspace_final_demo_matrix_v1.sh`
- payload invariant: inspect/query surfaces expose `read_path.filesystem_primary=false`.
