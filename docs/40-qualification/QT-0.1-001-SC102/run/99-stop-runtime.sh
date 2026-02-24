#!/usr/bin/env bash
set -euo pipefail
source "$(dirname "$0")/_lib.sh"

if [[ "$QT_MODE" == "live" ]]; then
  qt_live_env
  "$LIVE_RUN_DIR/99-stop-runtime.sh" || true
fi
