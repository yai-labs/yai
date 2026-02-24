#!/usr/bin/env bash
set -euo pipefail
DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
trap '"$DIR/99-stop-runtime.sh" || true' EXIT

"$DIR/01-start-runtime.sh"
"$DIR/02-create-workspace.sh"
"$DIR/03-start-target.sh"
"$DIR/04-run-curl.sh"
"$DIR/05-collect-evidence.sh"
"$DIR/06-assert-passfail.sh"

echo "RT-0.1-001-D1-EGRESS-CURL run completed: ${RUN_ID:-run-001}"
