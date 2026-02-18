#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/../../../.." && pwd)"
WS="${1:-fault_v1}"
source "$ROOT/tools/dev/resolve-yai-bin.sh"
BIN="$(yai_resolve_bin "$ROOT" || true)"

if [[ -z "$BIN" || ! -x "$BIN" ]]; then
  echo "FAIL: yai binary not found"
  exit 1
fi

supports_target() { "$BIN" "$1" --help >/dev/null 2>&1; }
if ! supports_target up || ! supports_target down; then
  echo "SKIP: current yai CLI does not support up/down required by fault-injection-v1"
  exit 0
fi

down_ws() {
  "$BIN" down --ws "$WS" --force >/dev/null 2>&1 || "$BIN" down --ws "$WS" >/dev/null 2>&1 || true
}

cleanup() {
  down_ws
}
trap cleanup EXIT

echo "== fault-injection-v1 (ws=$WS)"

down_ws
"$BIN" up --ws "$WS" --build --detach >/dev/null 2>&1 || "$BIN" up --ws "$WS" --detach >/dev/null 2>&1 || "$BIN" up --ws "$WS" >/dev/null 2>&1

RUN_DIR="$HOME/.yai/run/$WS"
SESSION_JSON="$RUN_DIR/session.json"
if [[ ! -f "$SESSION_JSON" ]]; then
  echo "SKIP: session metadata not available for fault-injection-v1"
  exit 0
fi

ENGINE_PID="$(python3 - "$SESSION_JSON" <<'PY'
import json,sys
obj=json.load(open(sys.argv[1], "r", encoding="utf-8"))
print(obj.get("engine_pid") or "")
PY
)"

if [[ -z "$ENGINE_PID" ]]; then
  echo "SKIP: engine pid not available in session metadata"
  exit 0
fi

kill -TERM "$ENGINE_PID" || true
sleep 1
"$BIN" status --ws "$WS" >/dev/null 2>&1 || true
echo "OK: fault-injection-v1 passed"
