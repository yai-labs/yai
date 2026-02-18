#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/../../.." && pwd)"
SPECS_CONTRACTS="$ROOT/deps/yai-specs/contracts"
FORMAL="$ROOT/deps/yai-specs/formal"
TLA_JAR="${TLA_JAR:-$HOME/Developer/tools/tla/tla2tools.jar}"

echo "=== CORE ROOT: $ROOT"
echo "=== CONTRACTS: $SPECS_CONTRACTS"
echo "=== FORMAL:    $FORMAL"
echo "=== TLA_JAR:   $TLA_JAR"

if [[ ! -f "$TLA_JAR" ]]; then
  echo "Missing TLA_JAR at $TLA_JAR. Set TLA_JAR or install tla2tools.jar."
  exit 1
fi

echo "=== CHECK GENERATED"
cd "$ROOT"
bash tools/dev/check-generated.sh

echo "=== UI NOTE"
echo "TUI removed from mind; UI verification moved to YX repo pipeline."

echo "=== CLI SPEC VALIDATION"
python3 - <<'PY'
import json, sys
from pathlib import Path

base = Path("deps/yai-specs/specs/cli/schema")
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

echo "=== COMPLIANCE BASELINE CHECK"
compliance_files=(
  "deps/yai-specs/contracts/extensions/compliance/C-001-compliance-context.md"
  "deps/yai-specs/compliance/schema/compliance.context.v1.json"
  "deps/yai-specs/compliance/packs/gdpr-eu/2026Q1/pack.meta.json"
  "deps/yai-specs/compliance/packs/gdpr-eu/2026Q1/taxonomy.data_classes.json"
  "deps/yai-specs/compliance/packs/gdpr-eu/2026Q1/taxonomy.purposes.json"
  "deps/yai-specs/compliance/packs/gdpr-eu/2026Q1/taxonomy.legal_basis.json"
)
for f in "${compliance_files[@]}"; do
  if [[ ! -f "$f" ]]; then
    echo "Missing compliance file: $f"
    exit 1
  fi
done

echo "=== TLC QUICK"
cd "$FORMAL"
java -XX:+UseParallelGC -jar "$TLA_JAR" -modelcheck tla/YAI_KERNEL.tla -config configs/YAI_KERNEL.quick.cfg

echo "=== TLC DEEP"
java -XX:+UseParallelGC -jar "$TLA_JAR" -modelcheck tla/YAI_KERNEL.tla -config configs/YAI_KERNEL.deep.cfg

echo "=== BUILD CORE"
cd "$ROOT"
make clean
make all

echo "OK: Core verification passed."
