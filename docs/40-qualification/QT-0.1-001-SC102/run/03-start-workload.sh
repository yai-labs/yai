#!/usr/bin/env bash
set -euo pipefail
source "$(dirname "$0")/_lib.sh"

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
