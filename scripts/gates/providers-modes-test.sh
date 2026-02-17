#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "$0")/../.." && pwd)"
source "$ROOT/scripts/dev/resolve-yai-bin.sh"
BIN="$(yai_resolve_bin "$ROOT" || true)"
WS="${1:-providers_modes_test}"
ENDPOINT="http://127.0.0.1:18080/v1/chat/completions?ws=${WS}"
PROVIDER_ID="remote:${ENDPOINT}"
if [[ -z "$BIN" || ! -x "$BIN" ]]; then
  echo "FAIL: yai binary not found"
  exit 1
fi

echo "== providers modes test (ws=$WS)"

"$BIN" down --ws "$WS" --force >/dev/null 2>&1 || true
"$BIN" up --ws "$WS" --build --no-engine --detach >/dev/null
YAI_REMOTE_ENDPOINT="$ENDPOINT" YAI_REMOTE_MODEL="qwen-test" \
  "$BIN" providers --ws "$WS" discover >/dev/null

"$BIN" providers trust --id "$PROVIDER_ID" --state revoked >/dev/null

OUT1="$(mktemp)"
./scripts/gates/providers.sh "$WS" >"$OUT1" 2>&1
grep -Fq "SKIP: no trusted provider (non-strict)" "$OUT1" || {
  echo "FAIL: non-strict mode did not skip on revoked provider"
  cat "$OUT1"
  exit 1
}

"$BIN" providers trust --id "$PROVIDER_ID" --state trusted >/dev/null
REQUIRE_ACTIVE_PROVIDER=1 ./scripts/gates/providers.sh "$WS" >/dev/null

"$BIN" providers trust --id "$PROVIDER_ID" --state revoked >/dev/null
set +e
REQUIRE_ACTIVE_PROVIDER=1 ./scripts/gates/providers.sh "$WS" >/tmp/providers_modes_strict_fail.txt 2>&1
RC=$?
set -e
if [[ "$RC" -eq 0 ]]; then
  echo "FAIL: strict mode should fail when provider is revoked"
  cat /tmp/providers_modes_strict_fail.txt
  exit 1
fi
grep -Fq "FAIL: no trusted provider (strict mode)" /tmp/providers_modes_strict_fail.txt || {
  echo "FAIL: strict failure reason missing"
  cat /tmp/providers_modes_strict_fail.txt
  exit 1
}

"$BIN" down --ws "$WS" --force >/dev/null 2>&1 || true
rm -f "$OUT1" /tmp/providers_modes_strict_fail.txt
echo "OK: providers modes test passed"
