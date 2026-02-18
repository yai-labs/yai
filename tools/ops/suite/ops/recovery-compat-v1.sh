#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/../../.." && pwd)"
WS="${1:-recovery_v1}"
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

up_ws() {
  "$BIN" up --ws "$WS" --detach >/dev/null && return 0
  sleep 1
  "$BIN" up --ws "$WS" --build --detach >/dev/null
}

echo "== recovery-compat-v1 (ws=$WS)"

"$BIN" down --ws "$WS" --force >/dev/null 2>&1 || true
up_ws

NODE_A="node:file:${WS}_a"
NODE_B="node:error:${WS}_b"
"$BIN" graph add-node --ws "$WS" --id "$NODE_A" --kind file --meta "{\"path\":\"${WS}.c\"}" >/dev/null
"$BIN" graph add-node --ws "$WS" --id "$NODE_B" --kind error --meta "{\"code\":\"E_REC\"}" >/dev/null
"$BIN" graph add-edge --ws "$WS" --src "$NODE_A" --dst "$NODE_B" --rel blocked_by_kernel --weight 1.0 >/dev/null

OUT1="$("$BIN" graph query --ws "$WS" --text "runtime sock" --k 8)"
echo "$OUT1" | rg -q "nodes:" || { echo "FAIL: query nodes missing before restart"; exit 1; }

"$BIN" down --ws "$WS" --force >/dev/null 2>&1 || true
up_ws
OUT2="$("$BIN" graph query --ws "$WS" --text "runtime sock" --k 8)"
echo "$OUT2" | rg -q "nodes:" || { echo "FAIL: query nodes missing after restart"; exit 1; }

# Compat pass: rebuild binaries without purging workspace state, then restart.
(cd "$ROOT" && make all >/dev/null)

"$BIN" down --ws "$WS" --force >/dev/null 2>&1 || true
up_ws
OUT3="$("$BIN" graph query --ws "$WS" --text "runtime sock" --k 8)"
echo "$OUT3" | rg -q "nodes:" || { echo "FAIL: query nodes missing after rebuild restart"; exit 1; }
echo "$OUT3" | rg -q "edges:" || { echo "FAIL: query edges missing after rebuild restart"; exit 1; }

echo "OK: recovery-compat-v1 passed"
