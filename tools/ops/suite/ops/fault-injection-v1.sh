#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/../../.." && pwd)"
WS="${1:-fault_v1}"
source "$ROOT/tools/dev/resolve-yai-bin.sh"
BIN="$(yai_resolve_bin "$ROOT" || true)"

if [[ -z "$BIN" || ! -x "$BIN" ]]; then
  echo "FAIL: yai binary not found"
  exit 1
fi

cleanup() {
  "$BIN" down --ws "$WS" --force >/dev/null 2>&1 || true
}
trap cleanup EXIT

echo "== fault-injection-v1 (ws=$WS)"

"$BIN" down --ws "$WS" --force >/dev/null 2>&1 || true
"$BIN" up --ws "$WS" --build --detach >/dev/null

RUN_DIR="$HOME/.yai/run/$WS"
SESSION_JSON="$RUN_DIR/session.json"
EVENTS_LOG="$RUN_DIR/events.log"
RUNTIME_SOCK="/tmp/yai_runtime_${WS}.sock"

ENGINE_PID="$(python3 - "$SESSION_JSON" <<'PY'
import json,sys
obj=json.load(open(sys.argv[1], "r", encoding="utf-8"))
print(obj.get("engine_pid") or "")
PY
)"
KERNEL_PID="$(python3 - "$SESSION_JSON" <<'PY'
import json,sys
obj=json.load(open(sys.argv[1], "r", encoding="utf-8"))
print(obj.get("kernel_pid") or "")
PY
)"

[[ -n "$ENGINE_PID" ]] || { echo "FAIL: missing engine pid"; exit 1; }
[[ -n "$KERNEL_PID" ]] || { echo "FAIL: missing kernel pid"; exit 1; }

kill -TERM "$ENGINE_PID"
sleep 1
STATUS_1="$("$BIN" status --ws "$WS")"
echo "$STATUS_1" | rg -q "engine:.*alive=false|halt_reason:" || {
  echo "FAIL: engine fault not reflected in status"
  exit 1
}

"$BIN" down --ws "$WS" --force >/dev/null 2>&1 || true
"$BIN" up --ws "$WS" --build --detach >/dev/null

KERNEL_PID="$(python3 - "$SESSION_JSON" <<'PY'
import json,sys
obj=json.load(open(sys.argv[1], "r", encoding="utf-8"))
print(obj.get("kernel_pid") or "")
PY
)"
[[ -n "$KERNEL_PID" ]] || { echo "FAIL: missing kernel pid after restart"; exit 1; }

kill -KILL "$KERNEL_PID"
for _ in 1 2 3 4 5; do
  [[ ! -e "$RUNTIME_SOCK" ]] && break
  sleep 1
done
if [[ -e "$RUNTIME_SOCK" ]]; then
  STATUS_2="$("$BIN" status --ws "$WS")"
  echo "$STATUS_2" | rg -q "halt_reason:.*kernel_dead|kernel:.*alive=false" || {
    echo "FAIL: kernel hard-fail not reflected in status"
    exit 1
  }
fi

"$BIN" down --ws "$WS" --force >/dev/null 2>&1 || true
"$BIN" down --ws "$WS" --force >/dev/null 2>&1 || true

if [[ -f "$EVENTS_LOG" ]]; then
  rg -q "proc_exit|kernel_dead|ws_down_complete" "$EVENTS_LOG" || {
    echo "FAIL: expected fault events not found in events.log"
    exit 1
  }
fi

echo "OK: fault-injection-v1 passed"
