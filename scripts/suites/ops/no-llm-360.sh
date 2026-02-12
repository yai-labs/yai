#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/../../.." && pwd)"
WS_PREFIX="${WS_PREFIX:-ops360}"
BIN="${BIN:-$(command -v yai || true)}"
if [[ -z "$BIN" && -x "$HOME/.cargo/bin/yai" ]]; then
  BIN="$HOME/.cargo/bin/yai"
fi
ITERATIONS="${ITERATIONS:-20}"
P95_BUDGET_MS="${P95_BUDGET_MS:-2000}"
SKIP_BASE="${SKIP_BASE:-0}"

if [[ -z "$BIN" || ! -x "$BIN" ]]; then
  if [[ -x "$ROOT/mind/target/release/yai" ]]; then
    BIN="$ROOT/mind/target/release/yai"
  fi
fi

if [[ -z "$BIN" || ! -x "$BIN" ]]; then
  echo "FAIL: yai binary not found"
  exit 1
fi

run() {
  echo
  echo ">>> $*"
  "$@"
}

export BIN

echo "== suite-ops-360-no-llm (ws_prefix=$WS_PREFIX)"

if [[ "$SKIP_BASE" != "1" ]]; then
  run bash -lc "cd \"$ROOT\" && DATASET_GATE=1 WS_PREFIX=\"${WS_PREFIX}\" ./scripts/suites/levels/l0-l7.sh"
fi
run bash -lc "cd \"$ROOT\" && P95_BUDGET_MS=\"$P95_BUDGET_MS\" ITERATIONS=\"$ITERATIONS\" ./scripts/suites/ops/perf-slo-v1.sh \"${WS_PREFIX}_perf\""
run bash -lc "cd \"$ROOT\" && ./scripts/suites/ops/fault-injection-v1.sh \"${WS_PREFIX}_fault\""
run bash -lc "cd \"$ROOT\" && ./scripts/suites/ops/security-sanity-v1.sh \"${WS_PREFIX}_sec\""
run bash -lc "cd \"$ROOT\" && ./scripts/suites/ops/recovery-compat-v1.sh \"${WS_PREFIX}_rec\""
run bash -lc "cd \"$ROOT\" && ITERATIONS=\"$ITERATIONS\" WS_PREFIX=\"${WS_PREFIX}_stress\" ./scripts/suites/ops/stress-v1.sh"

echo
echo "OK: suite-ops-360-no-llm passed"
