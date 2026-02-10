#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
LAW="$ROOT/law"
FORMAL="$LAW/formal"
TLA_JAR="${TLA_JAR:-/Users/francescomaiomascio/Developer/tools/tla/tla2tools.jar}"

echo "=== CORE ROOT: $ROOT"
echo "=== LAW:       $LAW"
echo "=== FORMAL:    $FORMAL"
echo "=== TLA_JAR:   $TLA_JAR"

echo "=== CHECK GENERATED"
cd "$ROOT"
bash scripts/check-generated.sh

echo "=== TLC QUICK"
cd "$FORMAL"
java -XX:+UseParallelGC -jar "$TLA_JAR" -modelcheck YAI_KERNEL.tla -config YAI_KERNEL.quick.cfg

echo "=== TLC DEEP"
java -XX:+UseParallelGC -jar "$TLA_JAR" -modelcheck YAI_KERNEL.tla -config YAI_KERNEL.deep.cfg

echo "=== BUILD CORE"
cd "$ROOT"
make clean
make all

echo "OK: Core verification passed."
