#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
source "$ROOT/tools/dev/resolve-yai-bin.sh"
BIN="$(yai_resolve_bin "$ROOT" || true)"
WS="events_test"
RUN_DIR="$HOME/.yai/run/${WS}"
SOCK="/tmp/yai_runtime_${WS}.sock"

if [[ -z "$BIN" || ! -x "$BIN" ]]; then
  echo "Missing yai binary in PATH"
  exit 1
fi

"$BIN" down --ws "$WS" || true
"$BIN" up --ws "$WS" --build --detach

TMP1="$(mktemp)"
TMP2="$(mktemp)"
"$BIN" events --ws "$WS" >"$TMP1" &
P1=$!
"$BIN" events --ws "$WS" >"$TMP2" &
P2=$!

sleep 1

ENGINE_PID=$(python3 - <<PY
import json, pathlib
p = pathlib.Path("$RUN_DIR/session.json")
print(json.loads(p.read_text()).get("engine_pid") or "")
PY
)
MIND_PID=$(python3 - <<PY
import json, pathlib
p = pathlib.Path("$RUN_DIR/session.json")
print(json.loads(p.read_text()).get("mind_pid") or "")
PY
)
KERNEL_PID=$(python3 - <<PY
import json, pathlib
p = pathlib.Path("$RUN_DIR/session.json")
print(json.loads(p.read_text()).get("kernel_pid") or "")
PY
)

if [[ -n "$ENGINE_PID" ]]; then kill -TERM "$ENGINE_PID"; fi
sleep 1
if [[ -n "$MIND_PID" ]]; then kill -TERM "$MIND_PID"; fi
sleep 1
if [[ -n "$KERNEL_PID" ]]; then kill -TERM "$KERNEL_PID"; fi
sleep 2

"$BIN" down --ws "$WS" || true

if [[ -e "$SOCK" ]]; then
  echo "FAIL: runtime socket still exists: $SOCK"
  exit 1
fi

kill "$P1" "$P2" >/dev/null 2>&1 || true

grep -q "proc_exit" "$TMP1" || { echo "FAIL: no proc_exit events in client1"; exit 1; }
grep -q "proc_exit" "$TMP2" || { echo "FAIL: no proc_exit events in client2"; exit 1; }

rm -f "$TMP1" "$TMP2"
echo "OK: verify-events passed"
