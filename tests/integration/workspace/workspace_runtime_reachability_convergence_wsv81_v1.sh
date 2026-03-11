#!/usr/bin/env bash
set -euo pipefail

YAI_ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/../../.." && pwd)"
CLI_ROOT="$YAI_ROOT/../cli"
LAW_ROOT="$YAI_ROOT/../law"
RUNTIME_BIN="$YAI_ROOT/build/bin/yai"
CLI_BIN="$CLI_ROOT/dist/bin/yai"
TMP="$(mktemp -d /tmp/yai-wsv81-convergence-XXXXXX)"
SOCK="/tmp/yai-wsv81-control.sock"
PIDF="/tmp/yai-wsv81-runtime.pid"
trap 'rm -rf "$TMP"; rm -f "$SOCK" "$PIDF"' EXIT

[[ -x "$RUNTIME_BIN" ]] || { echo "wsv81_convergence: missing runtime bin $RUNTIME_BIN"; exit 2; }
[[ -x "$CLI_BIN" ]] || { echo "wsv81_convergence: missing cli bin $CLI_BIN"; exit 2; }
[[ -d "$LAW_ROOT" ]] || { echo "wsv81_convergence: missing law repo $LAW_ROOT"; exit 2; }

export YAI_RUNTIME_INGRESS="$SOCK"
export YAI_RUNTIME_PIDFILE="$PIDF"
export YAI_SDK_COMPAT_REGISTRY_DIR="$LAW_ROOT"

rm -f "$SOCK" "$PIDF"
"$RUNTIME_BIN" up >"$TMP/runtime.log" 2>&1 &
RUNTIME_PID=$!

for _ in $(seq 1 80); do
  [[ -S "$SOCK" ]] && break
  sleep 0.1
done
[[ -S "$SOCK" ]] || { echo "wsv81_convergence: FAIL (runtime ingress socket missing: $SOCK)"; sed -n '1,160p' "$TMP/runtime.log"; kill "$RUNTIME_PID" >/dev/null 2>&1 || true; exit 1; }

set +e
"$CLI_BIN" lifecycle up >"$TMP/lifecycle_up.txt" 2>&1
RC_UP=$?
"$CLI_BIN" runtime ping >"$TMP/runtime_ping.txt" 2>&1
RC_PING=$?
"$CLI_BIN" ws status >"$TMP/ws_status.txt" 2>&1
RC_STATUS=$?
"$CLI_BIN" ws inspect >"$TMP/ws_inspect.txt" 2>&1
RC_INSPECT=$?
"$CLI_BIN" ws graph summary >"$TMP/ws_graph_summary.txt" 2>&1
RC_GRAPH=$?
set -e

kill "$RUNTIME_PID" >/dev/null 2>&1 || true
wait "$RUNTIME_PID" >/dev/null 2>&1 || true

if [[ "$RC_UP" -ne 0 ]]; then
  echo "wsv81_convergence: FAIL (lifecycle up rc=$RC_UP)"
  sed -n '1,120p' "$TMP/lifecycle_up.txt"
  exit 1
fi
if [[ "$RC_PING" -ne 0 ]]; then
  echo "wsv81_convergence: FAIL (runtime ping rc=$RC_PING)"
  sed -n '1,120p' "$TMP/runtime_ping.txt"
  exit 1
fi
if [[ "$RC_STATUS" -ne 0 ]]; then
  echo "wsv81_convergence: FAIL (ws status rc=$RC_STATUS)"
  sed -n '1,140p' "$TMP/ws_status.txt"
  exit 1
fi
if [[ "$RC_INSPECT" -ne 0 ]]; then
  echo "wsv81_convergence: FAIL (ws inspect rc=$RC_INSPECT)"
  sed -n '1,140p' "$TMP/ws_inspect.txt"
  exit 1
fi

# Rich family command must be non-dead: allow success or BAD_ARGS with no active workspace, but never SERVER_UNAVAILABLE.
if [[ "$RC_GRAPH" -eq 40 ]]; then
  echo "wsv81_convergence: FAIL (ws graph summary still dead rc=40)"
  sed -n '1,120p' "$TMP/ws_graph_summary.txt"
  exit 1
fi

rg -n "^runtime ping$|^OK$|Command completed\\." "$TMP/runtime_ping.txt" >/dev/null
rg -n "^Workspace status$|^Runtime capabilities$" "$TMP/ws_status.txt" >/dev/null
rg -n "^Workspace inspect$|^Runtime capabilities$" "$TMP/ws_inspect.txt" >/dev/null
if [[ "$RC_GRAPH" -eq 0 ]]; then
  rg -n "Workspace|graph|summary" "$TMP/ws_graph_summary.txt" >/dev/null
else
  rg -n "BAD ARGS|No active workspace selected for runtime execution" "$TMP/ws_graph_summary.txt" >/dev/null
fi

echo "wsv81_convergence: ok (up=$RC_UP ping=$RC_PING status=$RC_STATUS inspect=$RC_INSPECT graph=$RC_GRAPH)"
