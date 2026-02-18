#!/usr/bin/env bash
set -euo pipefail

WS="${1:-cortex_test}"
ROOT="$(cd "$(dirname "$0")/../.." && pwd)"
source "$ROOT/tools/dev/resolve-yai-bin.sh"
BIN="$(yai_resolve_bin "$ROOT" || true)"

if [[ ! -x "$BIN" ]]; then
  echo "FAIL: yai binary not found"
  exit 1
fi

(cd "$ROOT" && make all >/dev/null)
(cd "$ROOT/engine" && make test-cortex >/dev/null)

"$BIN" down --ws "$WS" --force >/dev/null 2>&1 || true

YAI_ENGINE_CORTEX_INITIAL_TARGET=4 \
YAI_ENGINE_CORTEX_UP_THRESHOLD=200 \
YAI_ENGINE_CORTEX_DOWN_THRESHOLD=100 \
YAI_ENGINE_CORTEX_DOWN_HOLD_MS=200 \
YAI_ENGINE_CORTEX_COOLDOWN_DOWN_MS=1000 \
"$BIN" up --ws "$WS" --build --detach >/dev/null

TMP_OUT="$(mktemp)"
("$BIN" events --ws "$WS" > "$TMP_OUT" & PID=$!; sleep 4; kill -INT "$PID" >/dev/null 2>&1 || true)

if ! rg -q "engine_scale_down" "$TMP_OUT"; then
  echo "FAIL: missing engine_scale_down in event stream"
  cat "$TMP_OUT" || true
  "$BIN" down --ws "$WS" --force >/dev/null 2>&1 || true
  exit 1
fi

if ! rg -q "\[YAI_CORTEX_EVENT\]" "$HOME/.yai/run/$WS/engine.log"; then
  echo "FAIL: missing cortex marker in engine.log"
  "$BIN" down --ws "$WS" --force >/dev/null 2>&1 || true
  exit 1
fi

if ! rg -q '"type":"engine_scale_down"' "$HOME/.yai/run/$WS/events.log"; then
  echo "FAIL: missing engine_scale_down in events.log"
  "$BIN" down --ws "$WS" --force >/dev/null 2>&1 || true
  exit 1
fi

"$BIN" down --ws "$WS" --force >/dev/null 2>&1 || true
rm -f "$TMP_OUT"

echo "OK: gate-cortex passed"
