#!/usr/bin/env bash
set -euo pipefail

WS="${1:-tui_gate}"
BIN="${BIN:-$(command -v yai || true)}"
if [[ -z "$BIN" && -x "$HOME/.cargo/bin/yai" ]]; then
  BIN="$HOME/.cargo/bin/yai"
fi

if [[ -z "$BIN" || ! -x "$BIN" ]]; then
  echo "ERR: yai binary not found" >&2
  exit 1
fi

echo "== tui gate (ws=$WS)"
"$BIN" down --ws "$WS" --force >/dev/null 2>&1 || true
"$BIN" up --ws "$WS" --build --detach >/dev/null

OUT1="/tmp/yai_tui_overview_${WS}.json"
OUT2="/tmp/yai_tui_graph_${WS}.json"
OUT3="/tmp/yai_tui_db_${WS}.json"

"$BIN" tui --ws "$WS" snapshot --view overview > "$OUT1"
"$BIN" tui --ws "$WS" snapshot --view graph > "$OUT2"
"$BIN" tui --ws "$WS" snapshot --view db > "$OUT3"

[[ -s "$OUT1" ]] || { echo "FAIL: overview snapshot empty"; exit 1; }
[[ -s "$OUT2" ]] || { echo "FAIL: graph snapshot empty"; exit 1; }
[[ -s "$OUT3" ]] || { echo "FAIL: db snapshot empty"; exit 1; }

rg -q '"ws"' "$OUT1" || { echo "FAIL: overview snapshot missing ws"; exit 1; }
rg -q '"payload"' "$OUT2" || { echo "FAIL: graph snapshot missing payload"; exit 1; }
rg -q '"payload"' "$OUT3" || { echo "FAIL: db snapshot missing payload"; exit 1; }

"$BIN" down --ws "$WS" --force >/dev/null 2>&1 || true

echo "OK: gate-tui passed"
