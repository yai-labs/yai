#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/../../../../" && pwd)"
OUT_DIR="$ROOT/build/test/unit_runtime"
mkdir -p "$OUT_DIR"

cc -Wall -Wextra -std=c11 -O2 \
  -I"$ROOT/include" \
  "$ROOT/tests/legacy/runtime/unit/test_runtime_state_closure.c" \
  "$ROOT/sys/policy/state/policy_state.c" \
  "$ROOT/sys/policy/state/grants_state.c" \
  "$ROOT/sys/policy/state/containment_state.c" \
  -o "$OUT_DIR/runtime_unit_tests"

"$OUT_DIR/runtime_unit_tests"
echo "runtime_unit_tests: ok"
