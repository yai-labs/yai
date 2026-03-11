#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/../../../.." && pwd)"
"$ROOT/tests/integration/source_plane/source_owner_ingest_bridge_v1.sh"
echo "ql_lan_enroll_attach_emit_v1: ok"
