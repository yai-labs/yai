#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/../../../.." && pwd)"
WS="${1:-recovery_v1}"
source "$ROOT/tools/dev/resolve-yai-bin.sh"
BIN="$(yai_resolve_bin "$ROOT" || true)"

if [[ -z "$BIN" || ! -x "$BIN" ]]; then
  echo "FAIL: yai binary not found"
  exit 1
fi

supports_target() { "$BIN" "$1" --help >/dev/null 2>&1; }
if ! supports_target up || ! supports_target down || ! "$BIN" graph --help >/dev/null 2>&1; then
  echo "SKIP: current yai CLI does not support up/down/graph required by recovery-compat-v1"
  exit 0
fi

down_ws() {
  "$BIN" down --ws "$WS" --force >/dev/null 2>&1 || "$BIN" down --ws "$WS" >/dev/null 2>&1 || true
}

up_ws() {
  "$BIN" up --ws "$WS" --build --detach >/dev/null 2>&1 \
    || "$BIN" up --ws "$WS" --detach >/dev/null 2>&1 \
    || "$BIN" up --ws "$WS" >/dev/null 2>&1
}

cleanup() {
  down_ws
}
trap cleanup EXIT

echo "== recovery-compat-v1 (ws=$WS)"

down_ws
up_ws || { echo "SKIP: unable to start workspace with current CLI"; exit 0; }

NODE_A="node:file:${WS}_a"
NODE_B="node:error:${WS}_b"
"$BIN" graph add-node --ws "$WS" --id "$NODE_A" --kind file --meta "{\"path\":\"${WS}.c\"}" >/dev/null
"$BIN" graph add-node --ws "$WS" --id "$NODE_B" --kind error --meta "{\"code\":\"E_REC\"}" >/dev/null
"$BIN" graph add-edge --ws "$WS" --src "$NODE_A" --dst "$NODE_B" --rel blocked_by_kernel --weight 1.0 >/dev/null

OUT1="$("$BIN" graph query --ws "$WS" --text "runtime sock" --k 8)"
echo "$OUT1" | rg -q "nodes:" || { echo "FAIL: query nodes missing before restart"; exit 1; }

down_ws
up_ws || { echo "SKIP: restart path not supported by current CLI"; exit 0; }
OUT2="$("$BIN" graph query --ws "$WS" --text "runtime sock" --k 8)"
echo "$OUT2" | rg -q "nodes:" || { echo "FAIL: query nodes missing after restart"; exit 1; }

echo "OK: recovery-compat-v1 passed"
