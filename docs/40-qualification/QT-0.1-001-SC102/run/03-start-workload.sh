#!/usr/bin/env bash
set -euo pipefail
source "$(dirname "$0")/_lib.sh"

if [[ "$QT_MODE" == "live" ]]; then
  qt_live_env
  "$LIVE_RUN_DIR/03-start-target.sh"
  python3 - <<'PY'
import json, os, datetime
state = {
  "workload_id": os.environ.get("WORKLOAD_ID", "wrk-d1-egress-live-v1"),
  "status": "running",
  "source": {"type": "real-target", "profile": os.environ.get("TARGET_PROFILE", "local")},
  "started_at": datetime.datetime.now(datetime.UTC).isoformat(),
}
open(os.path.join(os.environ["STATE_DIR"], "workload.json"), "w", encoding="utf-8").write(json.dumps(state, indent=2))
PY
  echo "workload started (live): $WORKLOAD_ID"
  exit 0
fi

python - <<"PY"
import json, os, datetime
meta_path = os.path.join(os.environ["QT_DIR"], "workload", "workload.meta.json")
with open(meta_path, "r", encoding="utf-8") as f:
    meta = json.load(f)
state = {
  "workload_id": os.environ["WORKLOAD_ID"],
  "status": "running",
  "source": meta,
  "started_at": datetime.datetime.utcnow().isoformat() + "Z",
}
path = os.path.join(os.environ["STATE_DIR"], "workload.json")
with open(path, "w", encoding="utf-8") as f:
    json.dump(state, f, indent=2)
PY

echo "workload started (simulated): $WORKLOAD_ID"
