#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
LAW="$ROOT/law"
FORMAL="$LAW/formal"
KERNEL="$ROOT/kernel"

TLA_JAR="${TLA_JAR:-/Users/francescomaiomascio/Developer/tools/tla/tla2tools.jar}"

echo "=== LAW ROOT: $LAW"
echo "=== KERNEL:   $KERNEL"
echo "=== FORMAL:   $FORMAL"
echo "=== TLA_JAR:  $TLA_JAR"

echo "=== TLC QUICK"
cd "$FORMAL"
java -XX:+UseParallelGC -jar "$TLA_JAR" -modelcheck YAI_KERNEL.tla -config YAI_KERNEL.quick.cfg

echo "=== TLC DEEP"
java -XX:+UseParallelGC -jar "$TLA_JAR" -modelcheck YAI_KERNEL.tla -config YAI_KERNEL.deep.cfg

echo "=== KERNEL BUILD"
cd "$KERNEL"
make clean
make

echo "OK: Law<->Kernel verification passed."
