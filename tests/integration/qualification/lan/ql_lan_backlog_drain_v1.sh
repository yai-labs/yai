#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/../../../.." && pwd)"
"$ROOT/tests/integration/source_plane/daemon_local_runtime_scan_spool_retry_v1.sh"
echo "ql_lan_backlog_drain_v1: ok"
