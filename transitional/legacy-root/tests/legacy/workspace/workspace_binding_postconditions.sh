#!/usr/bin/env bash
set -euo pipefail

YAI_ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/../../.." && pwd)"
RUNTIME_BIN="$YAI_ROOT/build/bin/yai"
CLI_BIN="$YAI_ROOT/build/bin/yai"
TMP="$(mktemp -d /tmp/yai-wsv82-postconditions-XXXXXX)"
SOCK="/tmp/yai-wsv82-control.sock"
PIDF="/tmp/yai-wsv82-runtime.pid"
WS="wsv82_bind_${RANDOM}"
trap 'rm -rf "$TMP"; rm -f "$SOCK" "$PIDF"' EXIT

[[ -x "$RUNTIME_BIN" ]] || { echo "wsv82_postconditions: missing runtime bin $RUNTIME_BIN"; exit 2; }
[[ -x "$CLI_BIN" ]] || { echo "wsv82_postconditions: missing cli bin $CLI_BIN"; exit 2; }

export YAI_RUNTIME_INGRESS="$SOCK"
export YAI_RUNTIME_PIDFILE="$PIDF"
export YAI_SDK_COMPAT_REGISTRY_DIR="$YAI_ROOT/governance"

rm -f "$SOCK" "$PIDF"
"$RUNTIME_BIN" up >"$TMP/runtime.log" 2>&1 &
RUNTIME_PID=$!

for _ in $(seq 1 80); do
  [[ -S "$SOCK" ]] && break
  sleep 0.1
done
[[ -S "$SOCK" ]] || { echo "wsv82_postconditions: FAIL (runtime ingress socket missing: $SOCK)"; sed -n '1,160p' "$TMP/runtime.log"; kill "$RUNTIME_PID" >/dev/null 2>&1 || true; exit 1; }

set +e
"$CLI_BIN" lifecycle up >"$TMP/lifecycle_up.txt" 2>&1
RC_UP=$?
"$CLI_BIN" ws create "$WS" >"$TMP/ws_create.txt" 2>&1
RC_CREATE=$?
"$CLI_BIN" ws set "$WS" >"$TMP/ws_set.txt" 2>&1
RC_SET=$?
"$CLI_BIN" ws open "$WS" >"$TMP/ws_open.txt" 2>&1
RC_OPEN=$?
"$CLI_BIN" ws current --json >"$TMP/ws_current.json" 2>&1
RC_CURRENT=$?
"$CLI_BIN" ws status --json >"$TMP/ws_status.json" 2>&1
RC_STATUS=$?
"$CLI_BIN" ws inspect --json >"$TMP/ws_inspect.json" 2>&1
RC_INSPECT=$?
"$CLI_BIN" ws db status >"$TMP/ws_db_status.txt" 2>&1
RC_DB=$?
set -e

kill "$RUNTIME_PID" >/dev/null 2>&1 || true
wait "$RUNTIME_PID" >/dev/null 2>&1 || true

for pair in \
  "lifecycle up:$RC_UP:$TMP/lifecycle_up.txt" \
  "ws create:$RC_CREATE:$TMP/ws_create.txt" \
  "ws set:$RC_SET:$TMP/ws_set.txt" \
  "ws open:$RC_OPEN:$TMP/ws_open.txt" \
  "ws current:$RC_CURRENT:$TMP/ws_current.json" \
  "ws status:$RC_STATUS:$TMP/ws_status.json" \
  "ws inspect:$RC_INSPECT:$TMP/ws_inspect.json" \
  "ws db status:$RC_DB:$TMP/ws_db_status.txt"
do
  IFS=: read -r name rc file <<<"$pair"
  if [[ "$rc" -ne 0 ]]; then
    echo "wsv82_postconditions: FAIL ($name rc=$rc)"
    sed -n '1,180p' "$file"
    exit 1
  fi
done

STORE_WS_ROOT="$HOME/.yai/run/data/$WS"
RUN_WS_ROOT="$HOME/.yai/run/$WS"

[[ -d "$STORE_WS_ROOT" ]] || { echo "wsv82_postconditions: FAIL (missing store workspace root $STORE_WS_ROOT)"; exit 1; }
[[ -d "$STORE_WS_ROOT/data" ]] || { echo "wsv82_postconditions: FAIL (missing data root $STORE_WS_ROOT/data)"; exit 1; }
[[ -d "$STORE_WS_ROOT/graph" ]] || { echo "wsv82_postconditions: FAIL (missing graph root $STORE_WS_ROOT/graph)"; exit 1; }
[[ -d "$STORE_WS_ROOT/knowledge" ]] || { echo "wsv82_postconditions: FAIL (missing knowledge root $STORE_WS_ROOT/knowledge)"; exit 1; }
[[ -d "$STORE_WS_ROOT/transient" ]] || { echo "wsv82_postconditions: FAIL (missing transient root $STORE_WS_ROOT/transient)"; exit 1; }

[[ -f "$RUN_WS_ROOT/manifest.json" ]] || { echo "wsv82_postconditions: FAIL (missing manifest $RUN_WS_ROOT/manifest.json)"; exit 1; }
[[ -f "$RUN_WS_ROOT/runtime/runtime-state.json" ]] || { echo "wsv82_postconditions: FAIL (missing runtime surface $RUN_WS_ROOT/runtime/runtime-state.json)"; exit 1; }
[[ -f "$RUN_WS_ROOT/metadata/binding.json" ]] || { echo "wsv82_postconditions: FAIL (missing binding surface $RUN_WS_ROOT/metadata/binding.json)"; exit 1; }
[[ -f "$RUN_WS_ROOT/state/workspace-state.json" ]] || { echo "wsv82_postconditions: FAIL (missing state surface $RUN_WS_ROOT/state/workspace-state.json)"; exit 1; }

rg -n "\"workspace_id\":\"$WS\"" "$TMP/ws_current.json" >/dev/null
rg -n "\"workspace_binding\":\\{\"selected\":true,\"bound\":true,\"workspace_id\":\"$WS\"\\}" "$TMP/ws_current.json" >/dev/null
rg -n "\"binding_status\":\"active\"" "$TMP/ws_status.json" >/dev/null
rg -n "\"containment_ready\":true" "$TMP/ws_status.json" >/dev/null
rg -n "\"runtime_attached\":true" "$TMP/ws_status.json" >/dev/null
rg -n "\"identity\":\\{\"workspace_id\":\"$WS\"" "$TMP/ws_inspect.json" >/dev/null
rg -n "\"runtime_capabilities\":\\{\"runtime\":\\{\"ready\":true" "$TMP/ws_inspect.json" >/dev/null
rg -n "Workspace bound[[:space:]]+yes" "$TMP/ws_db_status.txt" >/dev/null

echo "wsv82_postconditions: ok (ws=$WS store_root=$STORE_WS_ROOT)"
