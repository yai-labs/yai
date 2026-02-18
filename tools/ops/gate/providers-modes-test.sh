#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "$0")/../../.." && pwd)"
source "$ROOT/tools/dev/resolve-yai-bin.sh"
BIN="$(yai_resolve_bin "$ROOT" || true)"
WS="${1:-providers_modes_test}"
ENDPOINT="http://127.0.0.1:18080/v1/chat/completions?ws=${WS}"
PROVIDER_ID="remote:${ENDPOINT}"
if [[ -z "$BIN" || ! -x "$BIN" ]]; then
  echo "FAIL: yai binary not found"
  exit 1
fi

if ! "$BIN" providers --help >/dev/null 2>&1; then
  echo "SKIP: current yai CLI does not support target 'providers'"
  exit 0
fi

echo "== providers modes test (ws=$WS)"
echo "selected_provider_id=$PROVIDER_ID"

OUT1="$(mktemp)"
"$ROOT/tools/ops/gate/providers.sh" "$WS" >"$OUT1" 2>&1 || true
grep -Eq "SKIP:|FAIL: no trusted provider" "$OUT1" || {
  echo "FAIL: unexpected providers gate result"
  cat "$OUT1"
  exit 1
}

rm -f "$OUT1"
echo "OK: providers modes test passed"
