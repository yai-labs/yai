# Filesystem Decommission Walkthrough (DP-13)

## Purpose
Show how to classify a file-ish runtime artifact and map it to Data Plane truth + decommission action.

## Example 1: Transitional runtime state snapshot
- Artifact: `~/.yai/run/<ws>/workspace-state.json`
- Previous perception: operational truth for inspect/policy/debug.
- Current model: transitional summary file (`FS-C5 -> FS-C4`).
- Data Plane equivalent: event/evidence/governance/authority/artifact/enforcement/graph indexes and records.
- Action: keep temporarily, remove from operational path in DP-14/15 after binder/read closure.

## Example 2: Governance example pack in law
- Artifact: `law/manifests/customer-policy-packs/examples/*.json`
- Class: `FS-C2` (authoring example, not runtime truth).
- Action: keep as example/export surface, never runtime primary read source.

## Example 3: Transitional seed domain pack
- Artifact: `law/transitional/domain-family-seed/**`
- Class: `FS-C4` transitional residue.
- Action: move to `archive/data-plane-filesystem/` in DP-14.

## Verification check
After a governed action, inspect/query payloads must report:
- `read_path.mode = db_first`
- `read_path.filesystem_primary = false`

Script:
- `tests/integration/workspace/workspace_db_first_read_cutover.sh`
