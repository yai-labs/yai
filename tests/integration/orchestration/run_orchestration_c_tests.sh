#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/../../.." && pwd)"
OUT_ROOT="$ROOT/build/test/knowledge"
OBJ_DIR="$OUT_ROOT/obj"
BIN_DIR="$OUT_ROOT/bin"
mkdir -p "$OBJ_DIR" "$BIN_DIR"

if [[ ! -f "$OBJ_DIR/lib/runtime/lifecycle/runtime_capabilities.o" ]]; then
  "$ROOT/tests/unit/knowledge/run_knowledge_unit_tests.sh" >/dev/null
fi

OBJS=()
while IFS= read -r obj; do
  OBJS+=("$obj")
done < <(find "$OBJ_DIR/lib" -type f -name '*.o' | sort)

for t in test_runtime_primary test_orchestration_transport_smoke; do
  cc -Wall -Wextra -std=c11 -O2 -I"$ROOT/include" -I"$ROOT/include/yai" \
    "$ROOT/tests/integration/orchestration/$t.c" "${OBJS[@]}" -o "$BIN_DIR/$t" -lm
  "$BIN_DIR/$t"
done
