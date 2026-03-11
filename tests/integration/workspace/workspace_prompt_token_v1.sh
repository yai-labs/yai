#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/../../.." && pwd)"
TOKEN="$ROOT/tools/bin/yai-ws-token"

WS="ws_prompt_token_v1"
ALIAS="demo_prompt"
RUN_DIR="$HOME/.yai/run/$WS"
MANIFEST="$RUN_DIR/manifest.json"
WS_ROOT="$(mktemp -d)"
OUTSIDE_DIR="$(mktemp -d)"

cleanup() {
  rm -rf "$WS_ROOT" || true
  rm -rf "$OUTSIDE_DIR" || true
  rm -rf "$RUN_DIR" || true
}
trap cleanup EXIT

mkdir -p "$RUN_DIR"
cat >"$MANIFEST" <<EOF
{"type":"yai.workspace.manifest.v1","ws_id":"$WS","workspace_alias":"$ALIAS","root_path":"$WS_ROOT"}
EOF

# 1) outside workspace root -> empty token
pushd "$OUTSIDE_DIR" >/dev/null
out="$("$TOKEN")"
popd >/dev/null
[[ -z "$out" ]] || { echo "workspace_prompt_token_v1: FAIL (expected empty outside workspace root)"; exit 1; }

# 2) inside workspace root -> token present
mkdir -p "$WS_ROOT/subdir"
pushd "$WS_ROOT/subdir" >/dev/null
out="$("$TOKEN")"
popd >/dev/null
[[ "$out" == "◉ $ALIAS" ]] || { echo "workspace_prompt_token_v1: FAIL (unexpected token '$out')"; exit 1; }
[[ "$out" != ws:* ]] || { echo "workspace_prompt_token_v1: FAIL (token must not use ws: prefix)"; exit 1; }

# 3) missing manifest -> empty token
rm -f "$MANIFEST"
pushd "$WS_ROOT/subdir" >/dev/null
out="$("$TOKEN")"
popd >/dev/null
[[ -z "$out" ]] || { echo "workspace_prompt_token_v1: FAIL (expected empty token with missing manifest)"; exit 1; }

echo "workspace_prompt_token_v1: ok"
