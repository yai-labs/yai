#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/../../../.." && pwd)"
RUN_TS="$(date +%Y%m%d-%H%M%S)"
RUN_ID="${QW1_RUN_ID:-qw1-lan-$RUN_TS}"
EVID_ROOT="${QW1_EVIDENCE_ROOT:-$ROOT/tests/qualification/evidence/$RUN_ID}"

mkdir -p "$EVID_ROOT"
export YAI_QUAL_EVIDENCE_DIR="$EVID_ROOT"

PRECHECK=(
  "$ROOT/tests/legacy/qualification/lan/lan_command_contract.sh"
)

SCRIPTS=(
  "$ROOT/tests/legacy/qualification/lan/lan_enroll_attach_emit.sh"
  "$ROOT/tests/legacy/qualification/lan/lan_three_peers_same_workspace.sh"
  "$ROOT/tests/legacy/qualification/lan/lan_peer_offline_replay.sh"
  "$ROOT/tests/legacy/qualification/lan/lan_distinct_assets.sh"
  "$ROOT/tests/legacy/qualification/lan/lan_overlap_assets.sh"
  "$ROOT/tests/legacy/qualification/lan/lan_backlog_drain.sh"
)

echo "[QW-1/LAN] evidence_root=$EVID_ROOT"
printf "run_id=%s\nstarted_at=%s\n" "$RUN_ID" "$(date -u +%Y-%m-%dT%H:%M:%SZ)" >"$EVID_ROOT/meta.txt"

run_one() {
  local script="$1"
  local name
  local log
  local start
  local end
  local rc

  name="$(basename "$script")"
  log="$EVID_ROOT/$name.log"
  start="$(date +%s)"

  echo "[QW-1/LAN] running $name"
  if "$script" | tee "$log"; then
    rc=0
  else
    rc=$?
  fi

  end="$(date +%s)"
  printf "%s\trc=%s\tduration_s=%s\tlog=%s\n" "$name" "$rc" "$((end-start))" "$log" >>"$EVID_ROOT/results.tsv"

  if [[ "$rc" -ne 0 ]]; then
    echo "[QW-1/LAN] FAILED $name (rc=$rc), log=$log"
    exit "$rc"
  fi
}

for s in "${PRECHECK[@]}"; do
  run_one "$s"
done

for s in "${SCRIPTS[@]}"; do
  run_one "$s"
done

printf "finished_at=%s\n" "$(date -u +%Y-%m-%dT%H:%M:%SZ)" >>"$EVID_ROOT/meta.txt"
echo "run_lan_wave: ok"
