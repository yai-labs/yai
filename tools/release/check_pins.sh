#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
INFRA_ROOT_DEFAULT="$(cd "$ROOT/.." && pwd)/yai-infra"
INFRA_ROOT="${YAI_INFRA_ROOT:-$INFRA_ROOT_DEFAULT}"
INFRA_SCRIPT="$INFRA_ROOT/tools/release/check_pins.sh"
LEGACY_SCRIPT="$ROOT/tools/release/.legacy/check_pins.sh"

if [[ -z "${YAI_INFRA_DELEGATED:-}" && -x "$INFRA_SCRIPT" ]]; then
  cd "$ROOT"
  exec env YAI_INFRA_DELEGATED=1 "$INFRA_SCRIPT" "$@"
fi

if [[ -x "$LEGACY_SCRIPT" ]]; then
  cd "$ROOT"
  exec "$LEGACY_SCRIPT" "$@"
fi

echo "ERROR: local fallback unavailable for tools/release/check_pins.sh" >&2
exit 1
