#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/../../.." && pwd)"
OUT_DIR="$ROOT/build/test/unit_protocol"
CONTRACT_ROOT="${YAI_GOVERNANCE_CONTRACT_ROOT:-$ROOT/governance/contracts}"
if [[ ! -d "$CONTRACT_ROOT/protocol/include" ]]; then
  LAW_COMPAT_ROOT="${LAW_COMPAT_ROOT:-}"
  if [[ -z "$LAW_COMPAT_ROOT" ]]; then
    CANDIDATE="$(cd "$ROOT/.." && pwd)/law"
    [[ -d "$CANDIDATE" ]] && LAW_COMPAT_ROOT="$CANDIDATE"
  fi
  if [[ -z "$LAW_COMPAT_ROOT" ]]; then
    echo "contract root not found (expected governance/contracts or ../law/contracts)" >&2
    exit 2
  fi
  CONTRACT_ROOT="$LAW_COMPAT_ROOT/contracts"
fi
if [[ -z "${LAW_COMPAT_ROOT:-}" ]]; then
  CANDIDATE="$(cd "$ROOT/.." && pwd)/law"
  [[ -d "$CANDIDATE" ]] && LAW_COMPAT_ROOT="$CANDIDATE"
fi

mkdir -p "$OUT_DIR"

cc -Wall -Wextra -std=c11 -O2 \
  -I"$ROOT/include" -I"$ROOT/include/yai" \
  -I"$CONTRACT_ROOT/protocol/include" \
  -I"$CONTRACT_ROOT/protocol/runtime/include" \
  "$ROOT/tests/unit/protocol/protocol_test.c" \
  -o "$OUT_DIR/protocol_test"

"$OUT_DIR/protocol_test"
