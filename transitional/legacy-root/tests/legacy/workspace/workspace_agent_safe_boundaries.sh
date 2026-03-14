#!/usr/bin/env bash
set -euo pipefail

REPO="$(cd "$(dirname "${BASH_SOURCE[0]}")/../../.." && pwd)"

make -C "$REPO" governance-sync >/dev/null
python3 "$REPO/tools/validate/validate_agent_safe_primitives.py"
python3 "$REPO/tools/validate/validate_review_lifecycle.py"
"$REPO/tests/legacy/workspace/workspace_review_approval_gate.sh"

echo "workspace_agent_safe_boundaries_v1: ok"
