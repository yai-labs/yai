#!/usr/bin/env bash
set -euo pipefail

WS="${1:-graph_gate}"
BIN="${BIN:-$(command -v yai || true)}"
if [[ -z "$BIN" && -x "$HOME/.cargo/bin/yai" ]]; then
  BIN="$HOME/.cargo/bin/yai"
fi

if [[ -z "$BIN" && -x "$HOME/.yai/artifacts/yai-mind/target/release/yai" ]]; then
  BIN="$HOME/.yai/artifacts/yai-mind/target/release/yai"
fi

if [[ -z "$BIN" || ! -x "$BIN" ]]; then
  echo "ERR: yai binary not found" >&2
  exit 1
fi

echo "== graph gate (ws=$WS)"

SEED_A="node:gate:${WS}:a"
SEED_B="node:gate:${WS}:b"
SEED_C="node:gate:${WS}:c"

$BIN graph add-node --ws "$WS" --id "$SEED_A" --kind semantic --meta '{"seed":"a"}' >/dev/null
$BIN graph add-node --ws "$WS" --id "$SEED_B" --kind episodic --meta '{"seed":"b"}' >/dev/null
$BIN graph add-node --ws "$WS" --id "$SEED_C" --kind authority_policy --meta '{"seed":"c"}' >/dev/null
$BIN graph add-edge --ws "$WS" --src "$SEED_A" --dst "$SEED_B" --rel related --weight 1.0 >/dev/null
$BIN graph add-edge --ws "$WS" --src "$SEED_B" --dst "$SEED_C" --rel related --weight 1.0 >/dev/null

STATS_OUT="$($BIN graph stats --ws "$WS")"
echo "$STATS_OUT" | grep -q "^nodes: " || { echo "FAIL: stats missing nodes"; exit 1; }
echo "$STATS_OUT" | grep -q "^edges: " || { echo "FAIL: stats missing edges"; exit 1; }

OUT_DOT="/tmp/yai_graph_${WS}.dot"
$BIN graph export --ws "$WS" --format dot --out "$OUT_DOT" >/dev/null
[[ -s "$OUT_DOT" ]] || { echo "FAIL: dot export empty"; exit 1; }

NEI_OUT="$($BIN graph neighbors --ws "$WS" "$SEED_A" --depth 2)"
echo "$NEI_OUT" | grep -q "^nodes: " || { echo "FAIL: neighbors missing nodes"; exit 1; }
echo "$NEI_OUT" | grep -q "^edges: " || { echo "FAIL: neighbors missing edges"; exit 1; }
echo "$NEI_OUT" | grep -q "$SEED_A" || { echo "FAIL: neighbors missing seed"; exit 1; }

echo "OK: gate-graph passed"
