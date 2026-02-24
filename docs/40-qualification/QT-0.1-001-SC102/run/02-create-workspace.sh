#!/usr/bin/env bash
set -euo pipefail
source "$(dirname "$0")/_lib.sh"

if [[ "$QT_MODE" == "live" ]]; then
  qt_live_env
  "$LIVE_RUN_DIR/02-create-workspace.sh"
  echo "workspace context created (live): $RUN_ID"
  exit 0
fi

python - <<"PY"
import json, os, datetime
rid = os.environ["RUN_ID"]
ctx = {
  "ws_id": f"ws-sc102-{rid}",
  "trace_id": f"trace-sc102-{rid}",
  "principal": {"id": "principal-sc102", "role": "operator"},
  "arming_state": "armed",
  "created_at": datetime.datetime.utcnow().isoformat() + "Z",
}
path = os.path.join(os.environ["STATE_DIR"], "context.json")
with open(path, "w", encoding="utf-8") as f:
    json.dump(ctx, f, indent=2)
PY

echo "workspace context created: $RUN_ID"
