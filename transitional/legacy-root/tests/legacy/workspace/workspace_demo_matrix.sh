#!/usr/bin/env bash
set -euo pipefail

REPO="$(cd "$(dirname "${BASH_SOURCE[0]}")/../../.." && pwd)"

# This script composes the final matrix from existing validated scenarios.
"$REPO/tests/legacy/workspace/workspace_session_binding_contract.sh"
"$REPO/tests/legacy/workspace/workspace_inspect_surfaces.sh"
"$REPO/tests/legacy/workspace/workspace_real_flow.sh"
"$REPO/tests/legacy/workspace/workspace_scientific_flow.sh"
"$REPO/tests/legacy/workspace/workspace_digital_flow.sh"
"$REPO/tests/legacy/workspace/workspace_hostile_path_baseline.sh"
"$REPO/tests/legacy/workspace/workspace_isolation_guards.sh"
"$REPO/tests/legacy/workspace/workspace_negative_paths.sh"
"$REPO/tests/legacy/workspace/workspace_governed_vertical_slice.sh"
"$REPO/tests/legacy/workspace/workspace_governance_apply_semantics.sh"
"$REPO/tests/legacy/workspace/workspace_review_approval_gate.sh"
"$REPO/tests/legacy/workspace/workspace_agent_safe_boundaries.sh"
"$REPO/tests/legacy/workspace/workspace_event_evidence_sink_hardening.sh"
"$REPO/tests/legacy/workspace/workspace_governance_persistence.sh"
"$REPO/tests/legacy/workspace/workspace_authority_artifact_persistence.sh"
"$REPO/tests/legacy/workspace/workspace_brain_graph_transient.sh"
"$REPO/tests/legacy/workspace/workspace_operator_query_surfaces.sh"
"$REPO/tests/legacy/workspace/workspace_enforcement_record_closure.sh"
"$REPO/tests/legacy/workspace/workspace_graph_materialization_hooks.sh"
"$REPO/tests/legacy/workspace/workspace_db_first_read_cutover.sh"
"$REPO/tests/legacy/workspace/workspace_data_lifecycle_execution.sh"
"$REPO/tests/legacy/workspace/workspace_graph_read_surfaces.sh"

echo "workspace_final_demo_matrix_v1: ok"
