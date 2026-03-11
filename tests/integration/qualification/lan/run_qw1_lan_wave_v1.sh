#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/../../../.." && pwd)"

SCRIPTS=(
  "$ROOT/tests/integration/qualification/lan/ql_lan_enroll_attach_emit_v1.sh"
  "$ROOT/tests/integration/qualification/lan/ql_lan_three_peers_same_workspace_v1.sh"
  "$ROOT/tests/integration/qualification/lan/ql_lan_peer_offline_replay_v1.sh"
  "$ROOT/tests/integration/qualification/lan/ql_lan_distinct_assets_v1.sh"
  "$ROOT/tests/integration/qualification/lan/ql_lan_overlap_assets_v1.sh"
  "$ROOT/tests/integration/qualification/lan/ql_lan_backlog_drain_v1.sh"
)

for s in "${SCRIPTS[@]}"; do
  echo "[QW-1/LAN] running $(basename "$s")"
  "$s"
done

echo "run_qw1_lan_wave_v1: ok"
