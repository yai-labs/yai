#!/usr/bin/env bash
set -euo pipefail

WS="${1:-awareness_gate}"
BIN="${BIN:-$(command -v yai || true)}"
if [[ -z "$BIN" && -x "$HOME/.cargo/bin/yai" ]]; then
  BIN="$HOME/.cargo/bin/yai"
fi

if [[ -z "$BIN" || ! -x "$BIN" ]]; then
  echo "ERR: yai binary not found" >&2
  exit 1
fi

echo "== awareness gate (ws=$WS)"

"$BIN" graph add-node --ws "$WS" --id "node:awareness:${WS}:seed" --kind semantic --meta '{"seed":"awareness"}' >/dev/null
"$BIN" graph awareness --ws "$WS" --tick-ms 50 --max-steps 3 >/tmp/yai_awareness_gate.txt
"$BIN" graph stats --ws "$WS" >/tmp/yai_awareness_stats.txt

LOG="$HOME/.yai/run/$WS/awareness.log"
[[ -s "$LOG" ]] || { echo "FAIL: awareness.log is empty"; exit 1; }
rg -q "graph_stats" "$LOG" || { echo "FAIL: awareness intent not logged"; exit 1; }
rg -q "^nodes: " /tmp/yai_awareness_stats.txt || { echo "FAIL: graph stats missing nodes"; exit 1; }

echo "OK: gate-awareness passed"
