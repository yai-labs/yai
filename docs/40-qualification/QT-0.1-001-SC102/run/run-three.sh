#!/usr/bin/env bash
set -euo pipefail
DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"

for rid in run-001 run-002 run-003; do
  echo "=== $rid ==="
  RUN_ID="$rid" "$DIR/run-once.sh"
done

echo "QT-0.1-001-SC102 3/3 runs completed"
