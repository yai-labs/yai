#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/../../.." && pwd)"
SPECS_CONTRACTS="$ROOT/deps/yai-specs/contracts"
FORMAL="$ROOT/deps/yai-specs/formal"
KERNEL="$ROOT/kernel"

TLA_JAR="${TLA_JAR:-$HOME/Developer/tools/tla/tla2tools.jar}"

echo "=== CONTRACTS: $SPECS_CONTRACTS"
echo "=== KERNEL:    $KERNEL"
echo "=== FORMAL:    $FORMAL"
echo "=== TLA_JAR:   $TLA_JAR"

if [[ ! -f "$TLA_JAR" ]]; then
  echo "Missing TLA_JAR at $TLA_JAR. Set TLA_JAR or install tla2tools.jar."
  exit 1
fi

echo "=== GENERATE VAULT ABI"
cd "$ROOT"
./tools/dev/gen-vault-abi

echo "=== CHECK GENERATED"
bash tools/dev/check-generated.sh

echo "=== KERNEL BUILD"
cd "$KERNEL"
make clean
make

echo "=== TLC QUICK"
cd "$FORMAL"
java -XX:+UseParallelGC -jar "$TLA_JAR" -modelcheck tla/YAI_KERNEL.tla -config configs/YAI_KERNEL.quick.cfg

echo "=== TLC DEEP"
java -XX:+UseParallelGC -jar "$TLA_JAR" -modelcheck tla/YAI_KERNEL.tla -config configs/YAI_KERNEL.deep.cfg

echo "OK: Law<->Kernel verification passed."
