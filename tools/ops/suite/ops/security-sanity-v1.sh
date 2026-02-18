#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/../../.." && pwd)"
WS="${1:-security_v1}"
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

echo "== security-sanity-v1 (ws=$WS)"

"$BIN" down --ws "$WS" --force >/dev/null 2>&1 || true
"$BIN" up --ws "$WS" --build --detach >/dev/null

OUT1="$(mktemp)"
OUT2="$(mktemp)"
OUT3="$(mktemp)"

set +e
"$BIN" graph add-node --ws "$WS" --id "node:file:bad_meta" --kind file --meta '{invalid_json}' >"$OUT1" 2>&1
RC1=$?
"$BIN" providers --ws "$WS" attach "remote:http://127.0.0.1:1/v1/chat/completions" >"$OUT2" 2>&1
RC2=$?
"$BIN" dsar request nope --subject user1 >"$OUT3" 2>&1
RC3=$?
set -e

if [[ $RC2 -eq 0 ]]; then
  if rg -qi "attached" "$OUT2"; then
    echo "FAIL: attach unknown provider accepted"
    cat "$OUT2"
    exit 1
  fi
  rg -qi "error|not found|pair first|revoked|failed" "$OUT2" || {
    echo "FAIL: attach unknown provider returned success without explicit error"
    cat "$OUT2"
    exit 1
  }
fi
[[ $RC3 -ne 0 ]] || { echo "FAIL: invalid dsar request type accepted"; cat "$OUT3"; exit 1; }

# graph add-node meta may be accepted as opaque string by CLI; ensure it does not break query path.
"$BIN" graph query --ws "$WS" --text "runtime sock" --k 4 >/dev/null

cat "$OUT1" "$OUT2" "$OUT3" | rg -q "panicked at|thread 'main' panicked" && {
  echo "FAIL: panic detected on invalid input"
  cat "$OUT1" "$OUT2" "$OUT3"
  exit 1
}

rm -f "$OUT1" "$OUT2" "$OUT3"
echo "OK: security-sanity-v1 passed"
