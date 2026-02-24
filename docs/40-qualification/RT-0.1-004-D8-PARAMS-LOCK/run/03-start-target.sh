#!/usr/bin/env bash
set -euo pipefail
source "$(dirname "$0")/_lib.sh"

mkdir -p "$TARGET_PATH"

python3 - <<'PY'
import json, os
meta = {
  "target_type": "scientific.publish.path",
  "target_path": os.environ.get("TARGET_PATH"),
  "pipeline_id": os.environ.get("PIPELINE_ID"),
  "dataset_ref": os.environ.get("DATASET_REF"),
  "status": "ready",
}
open(os.path.join(os.environ["STATE_DIR"], "target.json"), "w", encoding="utf-8").write(json.dumps(meta, indent=2))
PY

echo "target ready: path=${TARGET_PATH}"
