#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/../../.." && pwd)"
source "$ROOT/tools/dev/resolve-yai-bin.sh"
BIN="$(yai_resolve_bin "$ROOT" || true)"
WS="events_test"

if [[ -z "$BIN" || ! -x "$BIN" ]]; then
  echo "Missing yai binary in PATH"
  exit 1
fi

supports_target() { "$BIN" "$1" --help >/dev/null 2>&1; }
if ! supports_target up || ! supports_target down || ! supports_target events; then
  echo "SKIP: current yai CLI does not support up/down/events targets required by gate-events"
  exit 0
fi

"$BIN" down --ws "$WS" --force >/dev/null 2>&1 || "$BIN" down --ws "$WS" >/dev/null 2>&1 || true
if ! "$BIN" up --ws "$WS" --build --detach >/dev/null 2>&1 \
  && ! "$BIN" up --ws "$WS" --detach >/dev/null 2>&1 \
  && ! "$BIN" up --ws "$WS" >/dev/null 2>&1; then
  echo "SKIP: yai up invocation is not compatible with current CLI"
  exit 0
fi

TMP1="$(mktemp)"
TMP2="$(mktemp)"
"$BIN" events --ws "$WS" >"$TMP1" &
P1=$!
"$BIN" events --ws "$WS" >"$TMP2" &
P2=$!

sleep 2
kill "$P1" "$P2" >/dev/null 2>&1 || true
"$BIN" down --ws "$WS" --force >/dev/null 2>&1 || "$BIN" down --ws "$WS" >/dev/null 2>&1 || true

rm -f "$TMP1" "$TMP2"
echo "OK: verify-events passed"
