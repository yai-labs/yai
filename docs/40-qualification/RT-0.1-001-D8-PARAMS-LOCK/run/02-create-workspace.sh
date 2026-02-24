#!/usr/bin/env bash
set -euo pipefail
source "$(dirname "$0")/_lib.sh"

python3 - <<'PY'
import datetime, json, os
ctx = {
  "ws_id": os.environ["WS_ID"],
  "trace_id": os.environ["TRACE_ID"],
  "principal": {"id": "principal-rt001d8", "role": "operator"},
  "arming_state": "armed",
  "created_at": datetime.datetime.now(datetime.UTC).isoformat(),
}
open(os.path.join(os.environ["STATE_DIR"], "context.json"), "w", encoding="utf-8").write(json.dumps(ctx, indent=2))
PY

echo "workspace context created: $RUN_ID"
