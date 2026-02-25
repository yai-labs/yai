#!/usr/bin/env bash
set -euo pipefail
DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"

for rid in run-001 run-002 run-003; do
  echo "=== $rid ==="
  RUN_ID="$rid" "$DIR/run-once.sh"
done

echo "RT-0.1-004-D1-GITHUB-ISSUE-COMMENT 3/3 runs completed"
