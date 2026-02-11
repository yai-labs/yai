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

echo "=== CLI SPEC VALIDATION"
python3 - <<'PY'
import json, sys
from pathlib import Path

base = Path("law/specs/cli")
schema = json.loads((base / "commands.schema.json").read_text())
data = json.loads((base / "commands.v1.json").read_text())

def fail(msg):
    print(f"CLI spec invalid: {msg}")
    sys.exit(1)

if "commands" not in data or not isinstance(data["commands"], list):
    fail("commands missing or not list")
for cmd in data["commands"]:
    for key in ("name", "group", "summary", "args"):
        if key not in cmd:
            fail(f"command missing {key}")
    if not isinstance(cmd["args"], list):
        fail("args not list")
print("OK: CLI spec schema checks passed")
PY

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
