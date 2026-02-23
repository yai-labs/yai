#!/usr/bin/env bash
set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
ROOT="$(git -C "$SCRIPT_DIR" rev-parse --show-toplevel 2>/dev/null || true)"
if [[ -z "$ROOT" ]]; then
  ROOT="$(cd "$SCRIPT_DIR/../../.." && pwd)"
fi

INFRA_ROOT_DEFAULT="$(cd "$ROOT/.." && pwd)/yai-infra"
INFRA_ROOT="${YAI_INFRA_ROOT:-$INFRA_ROOT_DEFAULT}"
TARGET="$INFRA_ROOT/tools/ops/suite/ops/fault-injection-v1.sh"

if [[ -x "$TARGET" ]]; then
  exec "$TARGET" "$@"
fi

echo "Deprecated local mirror: use infra canonical tool at tools/ops/suite/ops/fault-injection-v1.sh" >&2
echo "Missing target: $TARGET" >&2
exit 2
