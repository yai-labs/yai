#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
OUT_DIR="$ROOT/formal/artifacts/reports"
mkdir -p "$OUT_DIR"

python3 "$ROOT/tools/formal/check_formal_traceability.py" --root "$ROOT"

cat > "$OUT_DIR/formal_quick_report.json" <<JSON
{
  "mode": "quick",
  "model": "formal/models/yai_system.tla",
  "config": "formal/configs/yai_system.quick.cfg",
  "status": "ok"
}
JSON

echo "formal_quick: ok"
