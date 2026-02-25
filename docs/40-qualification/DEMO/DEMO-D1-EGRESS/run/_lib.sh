#!/usr/bin/env bash
set -euo pipefail

DEMO_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
REPO_ROOT="$(cd "$DEMO_DIR/../../.." && pwd)"
ARTIFACTS_DIR="$DEMO_DIR/artifacts"
CLI_DIR="$ARTIFACTS_DIR/cli"
LOG_DIR="$ARTIFACTS_DIR/logs"

mkdir -p "$CLI_DIR" "$LOG_DIR" "$ARTIFACTS_DIR/screenshots" "$ARTIFACTS_DIR/video"

cmd_capture() {
  local out="$1"
  shift
  {
    echo "$ $*"
    "$@"
  } >"$out" 2>&1 || true
}
