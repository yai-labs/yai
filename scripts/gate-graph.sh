#!/usr/bin/env bash
set -euo pipefail

WS="${1:-graph_gate}"
BIN="${BIN:-$(command -v yai || true)}"

if [[ -z "$BIN" && -x "$HOME/.yai/artifacts/yai-mind/target/release/yai" ]]; then
  BIN="$HOME/.yai/artifacts/yai-mind/target/release/yai"
fi

if [[ -z "$BIN" || ! -x "$BIN" ]]; then
  echo "ERR: yai binary not found" >&2
  exit 1
fi

echo "== graph gate (ws=$WS)"

$BIN embed --text "gate check" >/tmp/yai_gate_embed.txt

echo "$($BIN graph add-node --ws "$WS" --id "node:file:gate_a" --kind file --meta '{"path":"gate_a.c"}')" >/dev/null
echo "$($BIN graph add-node --ws "$WS" --id "node:error:gate_b" --kind error --meta '{"code":"E_GATE"}')" >/dev/null
echo "$($BIN graph add-edge --ws "$WS" --src "node:file:gate_a" --dst "node:error:gate_b" --rel blocked_by_kernel --weight 1.0)" >/dev/null

OUT=$($BIN graph query --ws "$WS" --text "runtime sock" --k 4)
echo "$OUT" | grep -q "nodes:" || { echo "FAIL: no nodes output"; exit 1; }
echo "$OUT" | grep -q "edges:" || { echo "FAIL: no edges output"; exit 1; }

echo "OK: gate-graph passed"
