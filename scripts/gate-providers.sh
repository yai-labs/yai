#!/usr/bin/env bash
set -euo pipefail

WS="${1:-providers_gate}"
ROOT_DIR="$(cd "$(dirname "$0")/.." && pwd)"
BIN=""
if [[ -x "$ROOT_DIR/mind/target/release/yai" ]]; then
  BIN="$ROOT_DIR/mind/target/release/yai"
fi
if [[ -z "${BIN}" ]]; then
  BIN="$(command -v yai || true)"
fi
if [[ -z "${BIN}" && -x "$HOME/.cargo/bin/yai" ]]; then
  BIN="$HOME/.cargo/bin/yai"
fi
if [[ -z "${BIN}" && -x "$HOME/.local/bin/yai" ]]; then
  BIN="$HOME/.local/bin/yai"
fi
if [[ -z "${BIN}" ]]; then
  echo "FAIL: yai not found in PATH"
  exit 1
fi

TRUST_FILE="$HOME/.yai/trust/providers.json"
RUN_FILE="$HOME/.yai/run/$WS/providers.json"
EVENTS_FILE="$HOME/.yai/run/$WS/events.log"

ENDPOINT="http://127.0.0.1:18080/v1/chat/completions?ws=${WS}"
PROVIDER_ID="remote:${ENDPOINT}"

echo "== providers gate (ws=$WS)"

"$BIN" down --ws "$WS" --force >/dev/null 2>&1 || true
"$BIN" up --ws "$WS" --no-engine --no-mind --detach >/dev/null

YAI_REMOTE_ENDPOINT="$ENDPOINT" \
YAI_REMOTE_MODEL="qwen-test" \
"$BIN" providers --ws "$WS" discover >/tmp/yai_providers_discover.txt 2>/tmp/yai_providers_discover.err

grep -q "candidate:" /tmp/yai_providers_discover.txt || { echo "FAIL: discover output"; cat /tmp/yai_providers_discover.err 2>/dev/null || true; cat /tmp/yai_providers_discover.txt 2>/dev/null || true; exit 1; }

"$BIN" providers --ws "$WS" pair "$PROVIDER_ID" "$ENDPOINT" "qwen-test" >/tmp/yai_providers_pair.txt
grep -q "paired" /tmp/yai_providers_pair.txt || { echo "FAIL: pair output"; exit 1; }

"$BIN" providers --ws "$WS" attach "$PROVIDER_ID" >/tmp/yai_providers_attach.txt
grep -q "attached" /tmp/yai_providers_attach.txt || { echo "FAIL: attach output"; exit 1; }

"$BIN" providers --ws "$WS" detach >/tmp/yai_providers_detach.txt
grep -q "detached" /tmp/yai_providers_detach.txt || { echo "FAIL: detach output"; exit 1; }

"$BIN" providers --ws "$WS" revoke "$PROVIDER_ID" >/tmp/yai_providers_revoke.txt
grep -q "revoked" /tmp/yai_providers_revoke.txt || { echo "FAIL: revoke output"; exit 1; }

"$BIN" providers --ws "$WS" attach "$PROVIDER_ID" >/tmp/yai_providers_attach_after_revoke.txt 2>&1 || true
if grep -q "attached" /tmp/yai_providers_attach_after_revoke.txt; then
  echo "FAIL: attach after revoke should fail"
  exit 1
fi
grep -qi "revoked\|not paired\|error:" /tmp/yai_providers_attach_after_revoke.txt || { echo "FAIL: expected revoke/paired error"; exit 1; }

[[ -f "$TRUST_FILE" ]] || { echo "FAIL: missing trust file"; exit 1; }
python - <<'PY' "$TRUST_FILE" "$PROVIDER_ID"
import json,sys
p=sys.argv[1]
pid=sys.argv[2]
obj=json.load(open(p))
assert obj.get("version")==1
prs=obj.get("providers",[])
rec=next((x for x in prs if x.get("id")==pid),None)
assert rec is not None
assert rec.get("trust_state")=="revoked"
assert "integrity" in rec
assert "audit" in rec
assert obj.get("integrity",{}).get("file_hash")
print("OK")
PY

[[ -f "$EVENTS_FILE" ]] || { echo "FAIL: missing events log"; exit 1; }
for ev in provider_discovered provider_paired provider_attached provider_detached provider_revoked; do
  grep -q "$ev" "$EVENTS_FILE" || { echo "FAIL: missing event $ev"; exit 1; }
done

"$BIN" down --ws "$WS" --force >/dev/null 2>&1 || true

if [[ -f "$RUN_FILE" ]]; then
  echo "FAIL: ws providers attachment file should be removed on down"
  exit 1
fi

echo "OK: gate-providers passed"
