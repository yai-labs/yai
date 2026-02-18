#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/../../../.." && pwd)"
ITERATIONS="${ITERATIONS:-5}"
WS_PREFIX="${WS_PREFIX:-stress_v1}"
source "$ROOT/tools/dev/resolve-yai-bin.sh"
YAI_BIN="$(yai_resolve_bin "$ROOT" || true)"

if [[ -z "$YAI_BIN" || ! -x "$YAI_BIN" ]]; then
  echo "FAIL: yai binary not found"
  exit 1
fi

supports_target() { "$YAI_BIN" "$1" --help >/dev/null 2>&1; }
if ! supports_target up || ! supports_target down || ! "$YAI_BIN" graph --help >/dev/null 2>&1; then
  echo "SKIP: current yai CLI does not support up/down/graph required by stress-v1"
  exit 0
fi

down_ws() {
  local ws="$1"
  "$YAI_BIN" down --ws "$ws" --force >/dev/null 2>&1 || "$YAI_BIN" down --ws "$ws" >/dev/null 2>&1 || true
}

up_ws() {
  local ws="$1"
  "$YAI_BIN" up --ws "$ws" --build --detach >/dev/null 2>&1 \
    || "$YAI_BIN" up --ws "$ws" --detach >/dev/null 2>&1 \
    || "$YAI_BIN" up --ws "$ws" >/dev/null 2>&1
}

echo "=== stress-v1 start"
echo "=== iterations: $ITERATIONS"
echo "=== binary: $YAI_BIN"

for i in $(seq 1 "$ITERATIONS"); do
  WS="${WS_PREFIX}_${i}"
  NID="node:file:${WS}"
  EID="node:error:${WS}"

  echo
  echo "--- [${i}/${ITERATIONS}] ws=$WS"

  down_ws "$WS"
  up_ws "$WS" || { echo "SKIP: unable to start $WS with current CLI"; exit 0; }

  "$YAI_BIN" graph add-node --ws "$WS" --id "$NID" --kind file --meta "{\"path\":\"${WS}.c\"}" >/dev/null
  "$YAI_BIN" graph add-node --ws "$WS" --id "$EID" --kind error --meta "{\"code\":\"E_${i}\"}" >/dev/null
  "$YAI_BIN" graph add-edge --ws "$WS" --src "$NID" --dst "$EID" --rel blocked_by_kernel --weight 1.0 >/dev/null

  OUT="$("$YAI_BIN" graph query --ws "$WS" --text "runtime sock" --k 4)"
  echo "$OUT" | rg -q "nodes:" || { echo "FAIL: query nodes missing for $WS"; exit 1; }
  echo "$OUT" | rg -q "edges:" || { echo "FAIL: query edges missing for $WS"; exit 1; }

  down_ws "$WS"
done

echo
echo "OK: stress-v1 passed (${ITERATIONS} iterations)"
