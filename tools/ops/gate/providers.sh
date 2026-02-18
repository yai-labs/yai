#!/usr/bin/env bash
set -euo pipefail

WS_RAW="${1:-providers_gate}"
WS="$WS_RAW"
if (( ${#WS_RAW} > 23 )); then
  WS="${WS_RAW:0:14}_${WS_RAW: -8}"
fi
ROOT_DIR="$(cd "$(dirname "$0")/../../.." && pwd)"
source "$ROOT_DIR/tools/dev/resolve-yai-bin.sh"
BIN="$(yai_resolve_bin "$ROOT_DIR" || true)"
REQUIRE_ACTIVE_PROVIDER="${REQUIRE_ACTIVE_PROVIDER:-0}"
TRUST_FILE="$HOME/.yai/trust/providers.json"

if [[ -z "$BIN" || ! -x "$BIN" ]]; then
  echo "FAIL: yai not found in PATH"
  exit 1
fi

if ! "$BIN" providers --help >/dev/null 2>&1; then
  echo "SKIP: current yai CLI does not support target 'providers'"
  exit 0
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
print(best.get('id',''))
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

echo "selected_provider_id=$SELECTED"
echo "SKIP: provider attach/status flow requires orchestration CLI not available in current environment"
exit 0
