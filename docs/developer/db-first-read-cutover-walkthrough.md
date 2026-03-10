# DB-First Read Cutover Walkthrough

## Goal
Show that inspect/query surfaces now read from persisted Data Plane records first, with explicit fallback markers.

## Scenario
1. Create and set a workspace.
2. Set digital domain/specialization.
3. Run a governed action (`digital.publish`).
4. Read surfaces:
   - `yai.workspace.inspect`
   - `yai.workspace.policy_effective`
   - `yai.workspace.debug_resolution`
   - `yai.workspace.query` for `events`, `evidence`, `governance`, `authority`, `artifacts`, `graph`, `enforcement`

## Expected
- Every response includes `read_path.mode = db_first`.
- Normal path: `db_first_ready=true`, `fallback_active=false`.
- Forced partial enforcement path: `fallback_active=true` and `fallback_reason` includes `enforcement_incomplete`.
- `filesystem_primary` is always `false`.

## Evidence script
Run:
- `tests/integration/workspace_lifecycle/workspace_db_first_read_cutover_dp12_v1.sh`
