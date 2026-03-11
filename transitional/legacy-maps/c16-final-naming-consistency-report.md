# C16.6 Final Naming Consistency Report

Date: 2026-03-11
Status: completed

## Scope
Final consistency sweep executed across root-facing and active operational surfaces:
- `tests/`
- `tools/`
- `docs/`
- validation/gating scripts and references

No architecture/topology refactor was introduced in this tranche.

## Residual Inconsistencies Found
1. Validator/tool naming mismatch after C16.5:
- `validate_overlay_compliance_runtime_view.py` still used `runtime_view` wording while paired generator was already canonicalized.

2. Active docs references pointing to non-canonical test paths:
- `tests/integration/workspace_lifecycle/...`
- `tests/integration/source_plane/...`

3. Script self-identification residue:
- `workspace_brain_graph_transient.sh` still emitted old `*_dp7_v1` labels.

## Applied Fixes
1. Tooling rename and reference cutover:
- `tools/validate/validate_overlay_compliance_runtime_view.py`
  -> `tools/validate/validate_overlay_compliance_view.py`
- Updated all references in tests/tools/docs/transitional notes.

2. Active docs path normalization:
- `tests/integration/workspace_lifecycle/...` -> `tests/integration/workspace/...`
- `tests/integration/source_plane/...` -> `tests/integration/source-plane/...`

3. Script label cleanup:
- Removed stale `dp7_v1` label strings from `workspace_brain_graph_transient.sh`.

## Final Naming Guardrails Added
New validator:
- `tools/validate/validate_final_naming_consistency.py`

Checks enforced:
- No version/task-shaped naming in active integration scripts (`_vN.sh`, `ql_`, `run_qw`, etc.).
- No legacy path tokens in active references (`workspace_lifecycle`, `source_plane`).
- No reintroduction of pre-cutover validator/generator names.

Integrated into:
- `tools/release/unified_repo_convergence_smoke.sh`
- `tools/bin/yai-governance-compat-check`
- `tests/unit/governance/run_governance_unit_tests.sh`

## Explicit Exceptions
- `transitional/` may still retain historical wording when used as legacy evidence/crosswalk material.
- Runtime/internal code symbols that are semantically correct but generic (e.g., domain-scoped `runtime_view`) were not force-renamed when not violating canonical grammar.

## Canonical Naming Rules Confirmed
1. Integration test scripts are scenario-based and non-milestone-shaped.
2. Validator/generator filenames use concise functional names.
3. Active docs reference only canonical current test paths.
4. Legacy naming is allowed only in explicitly historical/transitional contexts.
