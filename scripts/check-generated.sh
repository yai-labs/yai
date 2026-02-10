#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
SPEC="$ROOT/law/specs/vault/vault_abi.json"
GEN="$ROOT/scripts/gen-vault-abi.py"

TMP_DIR="$(mktemp -d)"
cleanup() {
  rm -rf "$TMP_DIR"
}
trap cleanup EXIT

python3 "$GEN" --spec "$SPEC" --out-dir "$TMP_DIR"

strip_generated() {
  sed -e '/^\/\* Generated:/d' -e '/^\\\* Generated:/d'
}

DIFF_A=$(diff -u <(strip_generated < "$ROOT/law/specs/vault/yai_vault_abi.h") \
                 <(strip_generated < "$TMP_DIR/law/specs/vault/yai_vault_abi.h") || true)
if [[ -n "$DIFF_A" ]]; then
  echo "ERROR: yai_vault_abi.h drift"
  echo "$DIFF_A"
  exit 1
fi

DIFF_B=$(diff -u <(strip_generated < "$ROOT/law/formal/law_ids.tla") \
                 <(strip_generated < "$TMP_DIR/law/formal/law_ids.tla") || true)
if [[ -n "$DIFF_B" ]]; then
  echo "ERROR: law_ids.tla drift"
  echo "$DIFF_B"
  exit 1
fi

echo "OK: generated files are in sync"
