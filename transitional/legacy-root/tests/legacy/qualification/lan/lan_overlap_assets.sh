#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/../../../.." && pwd)"
"$ROOT/tests/legacy/source-plane/source_plane_read_model.sh"
echo "lan_overlap_assets: ok"
