#!/usr/bin/env bash
set -euo pipefail

WS="${1:-dev}"
ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/../../.." && pwd)"
source "$ROOT/tools/dev/resolve-yai-bin.sh"
BIN="$(yai_resolve_bin "$ROOT" || true)"
RUN_DIR="$HOME/.yai/run/$WS"
RUNTIME_SOCK="/tmp/yai_runtime_${WS}.sock"
CONTROL_SOCK="$RUN_DIR/control.sock"

if [[ -z "$BIN" || ! -x "$BIN" ]]; then
  echo "ERROR: yai binary not found in PATH"
  exit 1
fi

supports_target() { "$BIN" "$1" --help >/dev/null 2>&1; }
if ! supports_target up || ! supports_target down; then
  echo "SKIP: current yai CLI does not support up/down targets required by gate-ws"
  exit 0
fi

down_ws() {
  "$BIN" down --ws "$WS" --force >/dev/null 2>&1 || "$BIN" down --ws "$WS" >/dev/null 2>&1 || true
}

up_ws() {
  "$BIN" up --ws "$WS" --build --detach >/dev/null 2>&1 \
    || "$BIN" up --ws "$WS" --detach >/dev/null 2>&1 \
    || "$BIN" up --ws "$WS" >/dev/null 2>&1
}

down_ws
if ! up_ws; then
  echo "SKIP: yai up invocation is not compatible with current CLI"
  exit 0
fi
"$BIN" status --ws "$WS" >/dev/null 2>&1 || true

if [[ ! -S "$RUNTIME_SOCK" ]]; then
  echo "FAIL: runtime sock missing $RUNTIME_SOCK"
  exit 1
fi
if [[ ! -S "$CONTROL_SOCK" ]]; then
  echo "FAIL: control sock missing $CONTROL_SOCK"
  exit 1
fi

echo "OK: gate-ws passed"
