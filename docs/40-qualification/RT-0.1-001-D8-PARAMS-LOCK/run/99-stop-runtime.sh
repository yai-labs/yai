#!/usr/bin/env bash
set -euo pipefail
source "$(dirname "$0")/_lib.sh"

"$YAI_BIN" down --ws "$WS_ID" --force >/dev/null 2>&1 || true

if [[ "$TARGET_PROFILE" == "docker" ]]; then
  docker compose -f "$DOCKER_COMPOSE_FILE" down >/dev/null || true
fi
