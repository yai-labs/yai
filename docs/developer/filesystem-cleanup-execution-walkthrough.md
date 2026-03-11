# Filesystem Cleanup Execution Walkthrough (DP-14)

## Before
- Transitional seed packs under `law/transitional/domain-family-seed/**` were present in operational repo tree.

## Why ambiguous
- They looked like policy/compliance state-bearing payloads despite being migration scaffolding.

## Execution
- Moved to `../archive/data-plane-filesystem/law/transitional/domain-family-seed/**`.
- Kept relocation map in:
  - `../archive/data-plane-filesystem/notes/RELOCATION_MAP.md`
  - `docs/program/reports/filesystem-cleanup-relocation-map.md`

## Runtime truth now
- Inspect/query surfaces remain DB-first and declare:
  - `read_path.mode = db_first`
  - `read_path.filesystem_primary = false`

## Verification
- `tests/integration/workspace/workspace_db_first_read_cutover.sh`
- `tests/integration/workspace/workspace_demo_matrix.sh`
