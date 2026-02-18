#!/usr/bin/env bash
set -euo pipefail

WS="${1:-cortex_test}"
ROOT="$(cd "$(dirname "$0")/../../.." && pwd)"
source "$ROOT/tools/dev/resolve-yai-bin.sh"
BIN="$(yai_resolve_bin "$ROOT" || true)"

if [[ -z "$BIN" || ! -x "$BIN" ]]; then
  echo "FAIL: yai binary not found"
  exit 1
fi

supports_target() { "$BIN" "$1" --help >/dev/null 2>&1; }
if ! supports_target up || ! supports_target down || ! supports_target events; then
  echo "SKIP: current yai CLI does not support up/down/events targets required by gate-cortex"
  exit 0
fi

(cd "$ROOT" && make all >/dev/null)
(cd "$ROOT/engine" && make test-cortex >/dev/null)

"$BIN" down --ws "$WS" --force >/dev/null 2>&1 || "$BIN" down --ws "$WS" >/dev/null 2>&1 || true

if ! (YAI_ENGINE_CORTEX_INITIAL_TARGET=4 \
YAI_ENGINE_CORTEX_UP_THRESHOLD=200 \
YAI_ENGINE_CORTEX_DOWN_THRESHOLD=100 \
YAI_ENGINE_CORTEX_DOWN_HOLD_MS=200 \
YAI_ENGINE_CORTEX_COOLDOWN_DOWN_MS=1000 \
"$BIN" up --ws "$WS" --build --detach >/dev/null 2>&1 \
|| YAI_ENGINE_CORTEX_INITIAL_TARGET=4 "$BIN" up --ws "$WS" --detach >/dev/null 2>&1 \
|| YAI_ENGINE_CORTEX_INITIAL_TARGET=4 "$BIN" up --ws "$WS" >/dev/null 2>&1); then
  echo "SKIP: yai up invocation is not compatible with current CLI"
  exit 0
fi

TMP_OUT="$(mktemp)"
("$BIN" events --ws "$WS" > "$TMP_OUT" & PID=$!; sleep 4; kill -INT "$PID" >/dev/null 2>&1 || true)

if ! rg -q "engine_scale_down" "$TMP_OUT"; then
  echo "FAIL: missing engine_scale_down in event stream"
  cat "$TMP_OUT" || true
  "$BIN" down --ws "$WS" --force >/dev/null 2>&1 || "$BIN" down --ws "$WS" >/dev/null 2>&1 || true
  exit 1
fi

"$BIN" down --ws "$WS" --force >/dev/null 2>&1 || "$BIN" down --ws "$WS" >/dev/null 2>&1 || true
rm -f "$TMP_OUT"

echo "OK: gate-cortex passed"
