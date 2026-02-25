#!/usr/bin/env bash
set -euo pipefail
DIR="$(cd "$(dirname "$0")/.." && pwd)"

required=(
  "$DIR/MANIFEST.json"
  "$DIR/RUNBOOK.md"
  "$DIR/CAPTURE.md"
  "$DIR/artifacts/cli/01-yai-version.txt"
)

for f in "${required[@]}"; do
  [[ -f "$f" ]] || { echo "FAIL: missing $f"; exit 1; }
done

echo "PASS: demo scaffolding verified"
