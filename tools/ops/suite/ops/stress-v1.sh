#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/../../.." && pwd)"
ITERATIONS="${ITERATIONS:-5}"
WS_PREFIX="${WS_PREFIX:-stress_v1}"
source "$ROOT/tools/dev/resolve-yai-bin.sh"
YAI_BIN="$(yai_resolve_bin "$ROOT" || true)"

if [[ -z "$YAI_BIN" || ! -x "$YAI_BIN" ]]; then
  echo "FAIL: yai binary not found"
  exit 1
fi

echo "=== stress-v1 start"
echo "=== iterations: $ITERATIONS"
echo "=== binary: $YAI_BIN"

for i in $(seq 1 "$ITERATIONS"); do
  WS="${WS_PREFIX}_${i}"
  RUN_DIR="$HOME/.yai/run/$WS"
  RUNTIME_SOCK="/tmp/yai_runtime_${WS}.sock"
  CONTROL_SOCK="$RUN_DIR/control.sock"
  NID="node:file:${WS}"
  EID="node:error:${WS}"

  echo
  echo "--- [${i}/${ITERATIONS}] ws=$WS"

  "$YAI_BIN" down --ws "$WS" --force >/dev/null 2>&1 || true
  "$YAI_BIN" up --ws "$WS" --build --detach
  "$YAI_BIN" status --ws "$WS" --json >/dev/null

  [[ -S "$RUNTIME_SOCK" ]] || { echo "FAIL: missing runtime sock for $WS"; exit 1; }
  [[ -S "$CONTROL_SOCK" ]] || { echo "FAIL: missing control sock for $WS"; exit 1; }

  "$YAI_BIN" graph add-node --ws "$WS" --id "$NID" --kind file --meta "{\"path\":\"${WS}.c\"}" >/dev/null
  "$YAI_BIN" graph add-node --ws "$WS" --id "$EID" --kind error --meta "{\"code\":\"E_${i}\"}" >/dev/null
  "$YAI_BIN" graph add-edge --ws "$WS" --src "$NID" --dst "$EID" --rel blocked_by_kernel --weight 1.0 >/dev/null

  OUT="$("$YAI_BIN" graph query --ws "$WS" --text "runtime sock" --k 4)"
  echo "$OUT" | rg -q "nodes:" || { echo "FAIL: query nodes missing for $WS"; exit 1; }
  echo "$OUT" | rg -q "edges:" || { echo "FAIL: query edges missing for $WS"; exit 1; }

  "$YAI_BIN" down --ws "$WS" --force >/dev/null 2>&1 || true
  [[ ! -e "$RUNTIME_SOCK" ]] || { echo "FAIL: runtime sock leak for $WS"; exit 1; }
  [[ ! -e "$CONTROL_SOCK" ]] || { echo "FAIL: control sock leak for $WS"; exit 1; }
done

echo
echo "OK: stress-v1 passed (${ITERATIONS} iterations)"
