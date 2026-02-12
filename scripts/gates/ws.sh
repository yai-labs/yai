#!/usr/bin/env bash
set -euo pipefail

WS="${1:-dev}"
BIN="${BIN:-$(command -v yai || true)}"
if [[ -z "$BIN" && -x "$HOME/.cargo/bin/yai" ]]; then
  BIN="$HOME/.cargo/bin/yai"
fi
RUN_DIR="$HOME/.yai/run/$WS"
RUNTIME_SOCK="/tmp/yai_runtime_${WS}.sock"
CONTROL_SOCK="$RUN_DIR/control.sock"

if [[ -z "$BIN" && -x "$HOME/.yai/artifacts/mind/target/release/yai" ]]; then
  BIN="$HOME/.yai/artifacts/mind/target/release/yai"
fi

if [[ -z "$BIN" || ! -x "$BIN" ]]; then
  echo "ERROR: yai binary not found in PATH"
  exit 1
fi

"$BIN" down --ws "$WS" --force || true
"$BIN" up --ws "$WS" --build --detach
"$BIN" status --ws "$WS"

if [[ ! -S "$RUNTIME_SOCK" ]]; then
  echo "FAIL: runtime sock missing $RUNTIME_SOCK"
  exit 1
fi
if [[ ! -S "$CONTROL_SOCK" ]]; then
  echo "FAIL: control sock missing $CONTROL_SOCK"
  exit 1
fi

KPID=$(ps aux | grep "[y]ai-kernel" | grep "$WS" | awk '{print $2}' | head -n1)
if [[ -z "$KPID" ]]; then
  echo "FAIL: no kernel pid found"
  exit 1
fi

lsof -nP -p "$KPID" | grep -q "$RUNTIME_SOCK" || {
  echo "FAIL: kernel pid not holding runtime sock"
  exit 1
}

DAEMON_PID=$(cat "$RUN_DIR/daemon.pid" 2>/dev/null || true)
if [[ -n "$DAEMON_PID" ]]; then
  lsof -nP -p "$DAEMON_PID" | grep -q control.sock || {
    echo "FAIL: daemon not holding control.sock"
    exit 1
  }
fi

kill -KILL "$KPID"
sleep 1
"$BIN" status --ws "$WS"

"$BIN" down --ws "$WS" --force || true

if [[ -e "$RUNTIME_SOCK" ]]; then
  echo "FAIL: runtime sock still exists"
  exit 1
fi
if [[ -e "$CONTROL_SOCK" ]]; then
  echo "FAIL: control sock still exists"
  exit 1
fi

ps aux | grep "[y]ai-kernel" | grep "$WS" && { echo "FAIL: kernel zombie"; exit 1; }

echo "OK: gate-ws passed"
