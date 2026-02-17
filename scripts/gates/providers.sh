#!/usr/bin/env bash
set -euo pipefail

WS_RAW="${1:-providers_gate}"
WS="$WS_RAW"
if (( ${#WS_RAW} > 23 )); then
  # Keep start + end so randomized suffixes survive while fitting shm name limits.
  WS="${WS_RAW:0:14}_${WS_RAW: -8}"
fi
ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
source "$ROOT_DIR/scripts/dev/resolve-yai-bin.sh"
BIN="$(yai_resolve_bin "$ROOT_DIR" || true)"
REQUIRE_ACTIVE_PROVIDER="${REQUIRE_ACTIVE_PROVIDER:-0}"
TRUST_FILE="$HOME/.yai/trust/providers.json"
RUN_DIR="$HOME/.yai/run/$WS"
if [[ -z "$BIN" || ! -x "$BIN" ]]; then
  echo "FAIL: yai not found in PATH"
  exit 1
fi

echo "== providers gate (ws=$WS, strict=$REQUIRE_ACTIVE_PROVIDER)"
if [[ "$WS" != "$WS_RAW" ]]; then
  echo "ws_normalized_from=$WS_RAW"
fi

SELECTED=""
if [[ -f "$TRUST_FILE" ]]; then
  SELECTED="$(python3 - "$TRUST_FILE" <<'PY'
import json,sys
p=sys.argv[1]
obj=json.load(open(p, 'r', encoding='utf-8'))
providers=obj.get('providers', [])
trusted=[]
for rec in providers:
    st=(rec.get('trust_state') or '').lower()
    if st in ('paired','attached','detached','trusted'):
        trusted.append(rec)
if not trusted:
    sys.exit(10)
trusted.sort(key=lambda r: int(r.get('last_seen') or 0), reverse=True)
best=trusted[0]
print("\t".join([
    best.get('id',''),
    best.get('endpoint',''),
    (best.get('trust_state') or '').lower(),
    str(best.get('last_seen') or 0),
]))
PY
  )" || true
fi

if [[ -z "$SELECTED" ]]; then
  if [[ "$REQUIRE_ACTIVE_PROVIDER" == "1" ]]; then
    echo "FAIL: no trusted provider (strict mode)"
    exit 1
  fi
  echo "SKIP: no trusted provider (non-strict)"
  exit 0
fi

IFS=$'\t' read -r SELECTED_ID SELECTED_ENDPOINT SELECTED_STATE SELECTED_LAST_SEEN <<<"$SELECTED"
echo "selected_provider_id=$SELECTED_ID"
echo "selected_provider_endpoint=$SELECTED_ENDPOINT"
echo "selected_provider_trust_state=$SELECTED_STATE"
echo "selected_provider_last_seen=$SELECTED_LAST_SEEN"

"$BIN" down --ws "$WS" --force >/dev/null 2>&1 || true
rm -rf "$RUN_DIR"

started=0
for attempt in 1 2 3 4 5; do
  if "$BIN" up --ws "$WS" --no-engine --detach >/dev/null 2>&1; then
    started=1
    break
  fi
  "$BIN" down --ws "$WS" --force >/dev/null 2>&1 || true
  rm -rf "$RUN_DIR"
  if "$BIN" up --ws "$WS" --build --no-engine --detach >/dev/null 2>&1; then
    started=1
    break
  fi
  sleep "$attempt"
done

if [[ "$started" -ne 1 ]]; then
  echo "FAIL: unable to start ws for providers gate after retries"
  exit 1
fi

ATTACH_OUT="$(mktemp)"
set +e
"$BIN" providers --ws "$WS" attach "$SELECTED_ID" >"$ATTACH_OUT" 2>&1
RC=$?
set -e

if [[ $RC -ne 0 ]]; then
  echo "FAIL: attach command failed for selected provider"
  cat "$ATTACH_OUT"
  "$BIN" down --ws "$WS" --force >/dev/null 2>&1 || true
  rm -f "$ATTACH_OUT"
  exit 1
fi

if ! rg -qi "attached|error:|not found|pair first|revoked|provider not paired" "$ATTACH_OUT"; then
  echo "FAIL: unexpected attach output"
  cat "$ATTACH_OUT"
  "$BIN" down --ws "$WS" --force >/dev/null 2>&1 || true
  rm -f "$ATTACH_OUT"
  exit 1
fi

if rg -qi "error:|not found|pair first|revoked|provider not paired" "$ATTACH_OUT"; then
  echo "FAIL: selected trusted provider could not be attached"
  cat "$ATTACH_OUT"
  "$BIN" down --ws "$WS" --force >/dev/null 2>&1 || true
  rm -f "$ATTACH_OUT"
  exit 1
fi

STATUS_OUT="$("$BIN" providers --ws "$WS" status 2>&1 || true)"
echo "$STATUS_OUT" | grep -Fq "active: $SELECTED_ID " || {
  echo "FAIL: provider status does not show selected provider active"
  echo "$STATUS_OUT"
  "$BIN" down --ws "$WS" --force >/dev/null 2>&1 || true
  rm -f "$ATTACH_OUT"
  exit 1
}

"$BIN" providers --ws "$WS" detach >/dev/null 2>&1 || true
"$BIN" down --ws "$WS" --force >/dev/null 2>&1 || true
rm -f "$ATTACH_OUT"

echo "OK: gate-providers passed"
