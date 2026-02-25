#!/usr/bin/env bash
set -euo pipefail
source "$(dirname "$0")/_lib.sh"

"$REPO_ROOT/tools/bin/yai-rt" down --ws "$WS_ID" --pids-json "$STATE_DIR/pids.json" >/dev/null 2>&1 || true

