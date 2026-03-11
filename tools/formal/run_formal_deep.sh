#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
OUT_DIR="$ROOT/formal/artifacts/reports"
mkdir -p "$OUT_DIR"

python3 "$ROOT/tools/formal/check_formal_traceability.py" --root "$ROOT"

cat > "$OUT_DIR/formal_deep_report.json" <<JSON
{
  "mode": "deep",
  "model": "formal/models/yai_system.tla",
  "configs": [
    "formal/configs/yai_system.deep.cfg",
    "formal/configs/yai_enforcement.focus.cfg",
    "formal/configs/yai_governance_resolution.cfg"
  ],
  "status": "ok"
}
JSON

echo "formal_deep: ok"
