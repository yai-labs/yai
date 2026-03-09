#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/../../.." && pwd)"
TOKEN="$ROOT/tools/bin/yai-ws-token"

WS="ws_prompt_token_v1"
ALIAS="demo_prompt"
SESSION_DIR="$HOME/.yai/session"
BIND_FILE="$SESSION_DIR/active_workspace.json"
RUN_DIR="$HOME/.yai/run/$WS"
MANIFEST="$RUN_DIR/manifest.json"
BACKUP="$(mktemp)"

cleanup() {
  if [[ -f "$BACKUP" ]]; then
    if [[ -f "$BIND_FILE" ]]; then
      rm -f "$BIND_FILE" || true
    fi
    if [[ -s "$BACKUP" ]]; then
      cp "$BACKUP" "$BIND_FILE"
    fi
    rm -f "$BACKUP" || true
  fi
  rm -rf "$RUN_DIR" || true
}
trap cleanup EXIT

mkdir -p "$SESSION_DIR"
if [[ -f "$BIND_FILE" ]]; then
  cp "$BIND_FILE" "$BACKUP"
fi
rm -f "$BIND_FILE"

# 1) no active workspace -> no token
out="$("$TOKEN")"
[[ -z "$out" ]] || { echo "workspace_prompt_token_v1: FAIL (expected empty without binding)"; exit 1; }

# 2) active binding + manifest -> token present
mkdir -p "$RUN_DIR"
cat >"$MANIFEST" <<EOF
{"type":"yai.workspace.manifest.v1","ws_id":"$WS"}
EOF
cat >"$BIND_FILE" <<EOF
{"type":"yai.workspace.binding.v1","workspace_id":"$WS","workspace_alias":"$ALIAS","bound_at":0,"source":"explicit"}
EOF

out="$("$TOKEN")"
[[ "$out" == "◉ $ALIAS" ]] || { echo "workspace_prompt_token_v1: FAIL (unexpected token '$out')"; exit 1; }
[[ "$out" != ws:* ]] || { echo "workspace_prompt_token_v1: FAIL (token must not use ws: prefix)"; exit 1; }

# 3) stale binding (missing manifest) -> no token
rm -f "$MANIFEST"
out="$("$TOKEN")"
[[ -z "$out" ]] || { echo "workspace_prompt_token_v1: FAIL (expected empty on stale binding)"; exit 1; }

echo "workspace_prompt_token_v1: ok"
