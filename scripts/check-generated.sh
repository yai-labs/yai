#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
SPEC="$ROOT/law/specs/vault/vault_abi.json"
GEN="$ROOT/scripts/gen-vault-abi"

TMP_DIR="$(mktemp -d)"
cleanup() {
  rm -rf "$TMP_DIR"
}
trap cleanup EXIT

"$GEN" --spec "$SPEC" --out-dir "$TMP_DIR"

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

SPEC_DUPES="$(find "$ROOT/docs/specs" -maxdepth 1 -type f -name "*.md" ! -name "README.md" ! -name "YAI_SOVEREIGN_ARCHITECTURE.md" -print)"
if [[ -n "$SPEC_DUPES" ]]; then
  while IFS= read -r spec_file; do
    [[ -z "$spec_file" ]] && continue
    if ! rg -q "^Canonical source(s)?:$|^Canonical sources:$|^Canonical source:$" "$spec_file"; then
      echo "ERROR: docs spec is not a canonical pointer: $spec_file"
      exit 1
    fi
  done <<<"$SPEC_DUPES"
fi
