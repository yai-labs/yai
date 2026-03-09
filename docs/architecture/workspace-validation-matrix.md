# Workspace Validation Matrix (WS-14 Final Demo Convergence)

This matrix defines the release-readiness checks for the workspace-driven normative path.

## Law

- `python3 tools/validate/validate_manifests.py`
- `python3 tools/validate/validate_domains.py`
- `python3 tools/validate/validate_compliance.py`
- `python3 tools/validate/validate_registry.py`

Expected: registry/contracts/manifests/domain corpus remain coherent with workspace command topology.

## YAI

- `make yai`
- `make test-law`
- integration workspace lifecycle scripts:
  - `tests/integration/workspace_lifecycle/workspace_runtime_contract_v1.sh`
  - `tests/integration/workspace_lifecycle/workspace_session_binding_contract_v1.sh`
  - `tests/integration/workspace_lifecycle/workspace_inspect_surfaces_v1.sh`
  - `tests/integration/workspace_lifecycle/workspace_real_flow_v1.sh`
  - `tests/integration/workspace_lifecycle/workspace_negative_paths_v1.sh`
  - `tests/integration/workspace_lifecycle/workspace_architecture_boundary_v1.sh`
  - `tests/integration/workspace_lifecycle/workspace_isolation_guards_v1.sh`
  - `tests/integration/workspace_lifecycle/workspace_containment_structure_v1.sh`
  - `tests/integration/workspace_lifecycle/workspace_security_envelope_v1.sh`
  - `tests/integration/workspace_lifecycle/workspace_execution_containment_hooks_v1.sh`
  - `tests/integration/workspace_lifecycle/workspace_hostile_path_baseline_v1.sh`
  - `tests/integration/workspace_lifecycle/workspace_scientific_flow_v1.sh`
  - `tests/integration/workspace_lifecycle/workspace_digital_flow_v1.sh`
  - `tests/integration/workspace_lifecycle/workspace_final_demo_matrix_v1.sh`
  - `tools/dev/verify_final_demo_matrix.sh`

Expected: lifecycle, vertical, and hostile-path scenarios converge as one governed workspace proof pack.

## CLI

- `make -j4 all`
- `YAI_SDK_COMPAT_REGISTRY_DIR=../law ./build/tests/unit_parse_test`
- `YAI_SDK_COMPAT_REGISTRY_DIR=../law tests/integration/help_guardrail.sh`
- `YAI_SDK_COMPAT_REGISTRY_DIR=../law tests/integration/porcelain_v1_guardrail.sh`

Expected: workspace grammar is parseable/discoverable and operator output remains disciplined.

## Notes

- In restricted sandbox environments, socket-binding integration tests may fail with `Operation not permitted`.
- In that case, run the same matrix in a local runtime-capable environment before release/demo signoff.
