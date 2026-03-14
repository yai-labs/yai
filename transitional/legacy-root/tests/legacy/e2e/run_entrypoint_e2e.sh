#!/usr/bin/env bash
set -euo pipefail

REPO="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
YAI="$REPO/build/bin/yai"
SOCK="${YAI_RUNTIME_INGRESS:-$HOME/.yai/run/control.sock}"

if [[ ! -x "$YAI" ]]; then
  make -C "$REPO" yai >/dev/null
fi

RUNTIME_PID=""
cleanup() {
  if [[ -n "$RUNTIME_PID" ]] && kill -0 "$RUNTIME_PID" 2>/dev/null; then
    kill "$RUNTIME_PID" >/dev/null 2>&1 || true
    wait "$RUNTIME_PID" >/dev/null 2>&1 || true
  fi
}
trap cleanup EXIT

"$YAI" --help >/dev/null

"$YAI" >/tmp/yai_e2e_up.log 2>&1 &
RUNTIME_PID=$!

for _ in $(seq 1 50); do
  if [[ -S "$SOCK" ]]; then
    break
  fi
  sleep 0.1
done

if [[ ! -S "$SOCK" ]]; then
  echo "run_entrypoint_e2e: FAIL (missing ingress socket $SOCK)"
  exit 1
fi

python3 "$REPO/tests/legacy/runtime/integration/test_handshake.py"

echo "run_entrypoint_e2e: ok"
