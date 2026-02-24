#!/usr/bin/env bash
set -euo pipefail
source "$(dirname "$0")/_lib.sh"

mkdir -p "$HOME/.yai/run/root" "$HOME/.yai/run/engine"
rm -f "$ROOT_SOCK" "$ENGINE_SOCK"

if [[ ! -x "$YAI_ROOT_BIN" || ! -x "$YAI_ENGINE_BIN" ]]; then
  make all >/dev/null
fi

YAI_EGRESS_ALLOWLIST="" YAI_PROVIDER_HOST="127.0.0.1" YAI_PROVIDER_PORT="8443" \
"$YAI_ENGINE_BIN" "$WS_ID" >"$ENGINE_LOG" 2>&1 &
ENGINE_PID=$!

wait_for_pid_alive "$ENGINE_PID" 10 || { echo "engine failed" >&2; exit 1; }
wait_for_socket "$ENGINE_SOCK" 20 || { echo "engine socket not ready" >&2; exit 1; }

"$YAI_ROOT_BIN" >"$ROOT_STDOUT_LOG" 2>"$ROOT_STDERR_LOG" &
ROOT_PID=$!

wait_for_pid_alive "$ROOT_PID" 10 || { echo "root failed" >&2; exit 1; }
wait_for_socket "$ROOT_SOCK" 20 || { echo "root socket not ready" >&2; exit 1; }

ENGINE_PID="$ENGINE_PID" ROOT_PID="$ROOT_PID" python3 - <<'PY'
import datetime, json, os
state = {
  "mode": "live",
  "started_at": datetime.datetime.now(datetime.UTC).isoformat(),
  "run_id": os.environ["RUN_ID"],
  "ws_id": os.environ["WS_ID"],
  "trace_id": os.environ["TRACE_ID"],
  "sockets": {"root": os.environ["ROOT_SOCK"], "engine": os.environ["ENGINE_SOCK"]},
}
open(os.path.join(os.environ["STATE_DIR"], "runtime.json"), "w", encoding="utf-8").write(json.dumps(state, indent=2))
open(os.path.join(os.environ["STATE_DIR"], "pids.json"), "w", encoding="utf-8").write(json.dumps({
  "engine_pid": int(os.environ["ENGINE_PID"]),
  "root_pid": int(os.environ["ROOT_PID"]),
}, indent=2))
PY

echo "runtime started (live): $RUN_ID"
