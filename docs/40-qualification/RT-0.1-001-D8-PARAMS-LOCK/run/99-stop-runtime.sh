#!/usr/bin/env bash
set -euo pipefail
source "$(dirname "$0")/_lib.sh"

python3 - <<'PY'
import json, os, signal
p = os.path.join(os.environ["STATE_DIR"], "pids.json")
try:
    data = json.load(open(p, "r", encoding="utf-8"))
except Exception:
    raise SystemExit(0)
for key in ("root_pid", "engine_pid"):
    pid = int(data.get(key, 0) or 0)
    if pid > 0:
        try:
            os.kill(pid, signal.SIGTERM)
        except Exception:
            pass
PY

sleep 0.2
rm -f "$ROOT_SOCK" "$ENGINE_SOCK"
if [[ "$TARGET_PROFILE" == "docker" ]]; then
  docker compose -f "$DOCKER_COMPOSE_FILE" down >/dev/null || true
fi
