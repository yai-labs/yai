#!/usr/bin/env bash
set -euo pipefail
source "$(dirname "$0")/_lib.sh"

if [[ "$QT_MODE" == "live" ]]; then
  qt_live_env
  "$LIVE_RUN_DIR/01-start-runtime.sh"
  echo "runtime started (live): $RUN_ID"
  exit 0
fi

python - <<"PY"
import json, os, datetime
state = {
  "runtime_mode": "simulated",
  "started_at": datetime.datetime.utcnow().isoformat() + "Z",
  "domain_pack_id": os.environ["DOMAIN_PACK_ID"],
  "baseline_id": os.environ["BASELINE_ID"],
  "run_id": os.environ["RUN_ID"],
}
path = os.path.join(os.environ["STATE_DIR"], "runtime.json")
with open(path, "w", encoding="utf-8") as f:
    json.dump(state, f, indent=2)
PY

echo "runtime started (simulated): $RUN_ID"
