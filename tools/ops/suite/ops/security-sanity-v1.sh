#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/../../../.." && pwd)"
WS="${1:-security_v1}"
source "$ROOT/tools/dev/resolve-yai-bin.sh"
BIN="$(yai_resolve_bin "$ROOT" || true)"

if [[ -z "$BIN" || ! -x "$BIN" ]]; then
  echo "FAIL: yai binary not found"
  exit 1
fi

supports_target() { "$BIN" "$1" --help >/dev/null 2>&1; }
if ! supports_target up || ! supports_target down || ! "$BIN" graph --help >/dev/null 2>&1; then
  echo "SKIP: current yai CLI does not support up/down/graph required by security-sanity-v1"
  exit 0
fi

down_ws() {
  "$BIN" down --ws "$WS" --force >/dev/null 2>&1 || "$BIN" down --ws "$WS" >/dev/null 2>&1 || true
}

cleanup() {
  down_ws
}
trap cleanup EXIT

echo "== security-sanity-v1 (ws=$WS)"

down_ws
"$BIN" up --ws "$WS" --build --detach >/dev/null 2>&1 || "$BIN" up --ws "$WS" --detach >/dev/null 2>&1 || "$BIN" up --ws "$WS" >/dev/null 2>&1

OUT1="$(mktemp)"
set +e
"$BIN" graph add-node --ws "$WS" --id "node:file:bad_meta" --kind file --meta '{invalid_json}' >"$OUT1" 2>&1
RC1=$?
set -e

cat "$OUT1" | rg -q "panicked at|thread 'main' panicked" && {
  echo "FAIL: panic detected on invalid input"
  cat "$OUT1"
  exit 1
}

rm -f "$OUT1"
if [[ $RC1 -eq 0 ]]; then
  echo "OK: security-sanity-v1 passed"
else
  echo "OK: security-sanity-v1 passed (invalid input rejected)"
fi
