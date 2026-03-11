#!/usr/bin/env bash
set -euo pipefail

REPO="$(cd "$(dirname "${BASH_SOURCE[0]}")/../../.." && pwd)"
LAW="$REPO/../law"

make -C "$REPO" law-embed-sync >/dev/null
python3 "$LAW/tools/validate/validate_agent_safe_primitives.py"
python3 "$LAW/tools/validate/validate_review_lifecycle.py"
"$REPO/tests/integration/workspace/workspace_review_approval_gate_v1.sh"

echo "workspace_agent_safe_boundaries_v1: ok"
