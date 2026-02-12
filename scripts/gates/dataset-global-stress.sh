#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
DATASET_DIR="$ROOT/datasets/global-stress/v1"
DATASET_SCRIPTS_DIR="$ROOT/scripts/datasets/global-stress/v1"
WS="${1:-dataset_stress}"
BIN="${BIN:-$(command -v yai || true)}"
if [[ -z "$BIN" && -x "$HOME/.cargo/bin/yai" ]]; then
  BIN="$HOME/.cargo/bin/yai"
fi

if [[ -z "$BIN" || ! -x "$BIN" ]]; then
  if [[ -x "$ROOT/mind/target/release/yai" ]]; then
    BIN="$ROOT/mind/target/release/yai"
  fi
fi

if [[ -z "$BIN" || ! -x "$BIN" ]]; then
  echo "FAIL: yai binary not found"
  exit 1
fi

if [[ ! -d "$DATASET_DIR" ]]; then
  echo "FAIL: dataset directory not found: $DATASET_DIR"
  exit 1
fi
if [[ ! -d "$DATASET_SCRIPTS_DIR" ]]; then
  echo "FAIL: dataset script directory not found: $DATASET_SCRIPTS_DIR"
  exit 1
fi

echo "== dataset gate (ws=$WS)"
echo "== dataset: $DATASET_DIR"

export BIN
export WS

bash "$DATASET_SCRIPTS_DIR/load-events-log.sh"
bash "$DATASET_SCRIPTS_DIR/import-seed-via-cli.sh"

OUT="$("$BIN" graph query --ws "$WS" --text "runtime sock" --k 8)"
echo "$OUT" | rg -q "nodes:" || { echo "FAIL: graph query nodes missing"; exit 1; }
echo "$OUT" | rg -q "edges:" || { echo "FAIL: graph query edges missing"; exit 1; }

echo "OK: gate-dataset-global-stress passed"
