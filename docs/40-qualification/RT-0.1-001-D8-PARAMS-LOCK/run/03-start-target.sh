#!/usr/bin/env bash
set -euo pipefail
source "$(dirname "$0")/_lib.sh"

if [[ "$TARGET_PROFILE" == "docker" ]]; then
  docker compose -f "$DOCKER_COMPOSE_FILE" ps >/dev/null
fi

mkdir -p "$TARGET_STORE_DIR"

python3 - <<'PY'
import json, os
meta = {
  "target_type": "scientific.publish.artifact_store",
  "target_profile": os.environ["TARGET_PROFILE"],
  "target_store_dir": os.environ["TARGET_STORE_DIR"],
  "target_url": os.environ["TARGET_URL"],
  "pipeline_id": os.environ["PIPELINE_ID"],
  "dataset_ref": os.environ["DATASET_REF"],
  "status": "ready",
}
open(os.path.join(os.environ["STATE_DIR"], "target.json"), "w", encoding="utf-8").write(json.dumps(meta, indent=2))
PY

echo "target ready: profile=${TARGET_PROFILE} url=${TARGET_URL}"
