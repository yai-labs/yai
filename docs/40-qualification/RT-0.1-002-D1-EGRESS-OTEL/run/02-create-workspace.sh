#!/usr/bin/env bash
set -euo pipefail
source "$(dirname "$0")/_lib.sh"

if [[ -z "$YAI_BIN" ]]; then
  echo "yai CLI not found in PATH" >&2
  exit 1
fi

# Ensure workspace lifecycle is governed by Root->Kernel path.
"$YAI_BIN" kernel ws destroy "$WS_ID" --arming --role operator >/dev/null 2>&1 || true
"$YAI_BIN" kernel ws create "$WS_ID" --arming --role operator >/dev/null

python3 - <<'PY'
import datetime, json, os
ctx = {
  "ws_id": os.environ["WS_ID"],
  "trace_id": os.environ["TRACE_ID"],
  "principal": {"id": "principal-rt002", "role": "operator"},
  "arming_state": "armed",
  "created_at": datetime.datetime.now(datetime.UTC).isoformat(),
}
open(os.path.join(os.environ["STATE_DIR"], "context.json"), "w", encoding="utf-8").write(json.dumps(ctx, indent=2))
PY

echo "workspace context created: $RUN_ID"
