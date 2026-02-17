#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/../../.." && pwd)"
WS_PREFIX="${WS_PREFIX:-l7}"
source "$ROOT/scripts/dev/resolve-yai-bin.sh"
YAI_BIN="$(yai_resolve_bin "$ROOT" || true)"
DATASET_GATE="${DATASET_GATE:-0}"

run() {
  echo
  echo ">>> $*"
  "$@"
}

step() {
  echo
  echo "=============================="
  echo "== $1"
  echo "=============================="
}

step "L0 - Canonical Sources + Legacy Name Scan"
run bash -lc "cd \"$ROOT\" && ./scripts/gen-vault-abi"
run bash -lc "cd \"$ROOT\" && ./scripts/check-generated.sh"
run bash -lc "cd \"$ROOT\" && if rg -n \"Ice|ICE_\" boot root kernel engine runtime scripts; then echo \"FAIL: legacy Ice/ICE symbols found\"; exit 1; else echo \"OK: no Ice/ICE legacy symbols\"; fi"

if [[ -z "$YAI_BIN" || ! -x "$YAI_BIN" ]]; then
  echo "FAIL: yai binary not found"
  exit 1
fi

export BIN="$YAI_BIN"
export YAI_BIN

step "L1 - Law <-> Kernel Formal + Build"
run bash -lc "cd \"$ROOT\" && ./scripts/verify/law-kernel.sh"

step "L2 - Core Verify (TLA + build + compliance baseline)"
run bash -lc "cd \"$ROOT\" && ./scripts/verify/core.sh"

step "L3 - Workspace Lifecycle Gate"
run bash -lc "cd \"$ROOT\" && ./scripts/gates/ws.sh \"${WS_PREFIX}_ws\""

step "L4 - Cortex Determinism Gate"
run bash -lc "cd \"$ROOT\" && ./scripts/gates/cortex.sh \"${WS_PREFIX}_cortex\""

step "L5 - Event Stream Reliability Gate"
run bash -lc "cd \"$ROOT\" && ./scripts/gates/events.sh"

step "L6 - Graph Gate"
run bash -lc "cd \"$ROOT\" && ./scripts/gates/graph.sh \"${WS_PREFIX}_graph\""



step "L7 - Providers + Rust Unit/Integration Tests + CLI Smoke"
PROVIDERS_WS="${WS_PREFIX}_prv_$RANDOM"
run bash -lc "cd \"$ROOT\" && ./scripts/gates/providers.sh \"${PROVIDERS_WS}\""
run bash -lc "cd \"$ROOT\" && \"$YAI_BIN\" test smoke --ws \"${WS_PREFIX}_smoke\" --timeout-ms 8000"

if [[ "$DATASET_GATE" == "1" ]]; then
  step "L7b - Dataset Global Stress Seed Gate"
  run bash -lc "cd \"$ROOT\" && BIN=\"$YAI_BIN\" ./scripts/gates/dataset-global-stress.sh \"${WS_PREFIX}_dataset\""
fi

echo
echo "OK: suite L0..L7 passed"
