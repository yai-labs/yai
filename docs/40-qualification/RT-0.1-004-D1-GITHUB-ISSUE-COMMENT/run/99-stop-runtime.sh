#!/usr/bin/env bash
set -euo pipefail
source "$(dirname "$0")/_lib.sh"

"$YAI_BIN" down --ws "$WS_ID" --force >/dev/null 2>&1 || true

