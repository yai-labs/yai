#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/../../../.." && pwd)"
"$ROOT/tests/integration/source-plane/source_plane_read_model_v1.sh"
echo "ql_lan_overlap_assets_v1: ok"
