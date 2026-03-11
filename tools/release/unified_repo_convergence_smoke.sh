#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"

run() {
  echo "[b13-smoke] $*"
  "$@"
}

run make -C "$ROOT" governance-check
run python3 "$ROOT/tools/validate/validate_root_framing.py"
run python3 "$ROOT/tools/validate/validate_tooling_legacy_refs.py"
run python3 "$ROOT/tools/validate/validate_governance_manifests.py"
run python3 "$ROOT/tools/validate/validate_aux_naming.py"
run python3 "$ROOT/tools/validate/validate_governance_contracts_schema.py"
run python3 "$ROOT/tools/validate/validate_governance_ingestion_pipeline.py"
run "$ROOT/tests/unit/governance/run_governance_unit_tests.sh"
run "$ROOT/tests/integration/governance/run_governance_resolution_smoke.sh"

echo "[b13-smoke] unified repo convergence: OK"
