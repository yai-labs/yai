#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/../../../../.." && pwd)"
OUT_DIR="$ROOT/build/test/unit_providers"
mkdir -p "$OUT_DIR"

cc -Wall -Wextra -std=c11 -O2 \
  -I"$ROOT/include" -I"$ROOT/third_party/cjson" \
  "$ROOT/tests/legacy/unit/network/providers/test_registry_selection.c" \
  "$ROOT/sys/network/providers/provider_registry.c" \
  "$ROOT/sys/network/providers/provider_policy.c" \
  "$ROOT/sys/network/providers/provider_selection.c" \
  "$ROOT/third_party/cjson/cJSON.c" \
  -o "$OUT_DIR/providers_unit_tests"

"$OUT_DIR/providers_unit_tests"
echo "providers_unit_tests: ok"
