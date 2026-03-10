#!/usr/bin/env bash
set -euo pipefail

REPO="$(cd "$(dirname "${BASH_SOURCE[0]}")/../../.." && pwd)"

# This script composes the final matrix from existing validated scenarios.
"$REPO/tests/integration/workspace_lifecycle/workspace_session_binding_contract_v1.sh"
"$REPO/tests/integration/workspace_lifecycle/workspace_inspect_surfaces_v1.sh"
"$REPO/tests/integration/workspace_lifecycle/workspace_real_flow_v1.sh"
"$REPO/tests/integration/workspace_lifecycle/workspace_scientific_flow_v1.sh"
"$REPO/tests/integration/workspace_lifecycle/workspace_digital_flow_v1.sh"
"$REPO/tests/integration/workspace_lifecycle/workspace_hostile_path_baseline_v1.sh"
"$REPO/tests/integration/workspace_lifecycle/workspace_isolation_guards_v1.sh"
"$REPO/tests/integration/workspace_lifecycle/workspace_negative_paths_v1.sh"
"$REPO/tests/integration/workspace_lifecycle/workspace_governed_vertical_slice_v1.sh"
"$REPO/tests/integration/workspace_lifecycle/workspace_governance_apply_semantics_v1.sh"
"$REPO/tests/integration/workspace_lifecycle/workspace_review_approval_gate_v1.sh"
"$REPO/tests/integration/workspace_lifecycle/workspace_agent_safe_boundaries_v1.sh"
"$REPO/tests/integration/workspace_lifecycle/workspace_event_evidence_sink_hardening_v1.sh"
"$REPO/tests/integration/workspace_lifecycle/workspace_governance_persistence_dp5_v1.sh"
"$REPO/tests/integration/workspace_lifecycle/workspace_authority_artifact_persistence_dp6_v1.sh"
"$REPO/tests/integration/workspace_lifecycle/workspace_brain_graph_transient_dp7_v1.sh"
"$REPO/tests/integration/workspace_lifecycle/workspace_operator_query_surfaces_dp8_v1.sh"
"$REPO/tests/integration/workspace_lifecycle/workspace_enforcement_record_closure_dp10_v1.sh"
"$REPO/tests/integration/workspace_lifecycle/workspace_graph_materialization_hooks_dp11_v1.sh"
"$REPO/tests/integration/workspace_lifecycle/workspace_db_first_read_cutover_dp12_v1.sh"
"$REPO/tests/integration/workspace_lifecycle/workspace_data_lifecycle_execution_dp15b_v1.sh"
"$REPO/tests/integration/workspace_lifecycle/workspace_graph_read_surfaces_dp16_v1.sh"

echo "workspace_final_demo_matrix_v1: ok"
