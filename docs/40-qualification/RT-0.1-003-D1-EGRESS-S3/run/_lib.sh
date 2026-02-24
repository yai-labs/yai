#!/usr/bin/env bash
set -euo pipefail

RT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
REPO_ROOT="$(cd "$RT_DIR/../../.." && pwd)"

DOMAIN_PACK_ID="${DOMAIN_PACK_ID:-D1-digital/egress-v1}"
BASELINE_ID="${BASELINE_ID:-baseline-deny}"
RUN_ID="${RUN_ID:-run-001}"
WORKLOAD_ID="${WORKLOAD_ID:-wrk-d1-egress-s3-v1}"
ATTACK_PROFILE_ID="${ATTACK_PROFILE_ID:-rt-003-s3-upload-egress-attempt}"

PACK_DIR="$REPO_ROOT/docs/30-catalog/domains/packs/$DOMAIN_PACK_ID"
BASELINE_FILE="$PACK_DIR/contracts/${BASELINE_ID}.json"
EXPECTED_FILE="$PACK_DIR/vectors/expected_outcomes.json"
EVIDENCE_DIR="$RT_DIR/evidence/$DOMAIN_PACK_ID/$RUN_ID"
STATE_DIR="$RT_DIR/run/.state/$RUN_ID"

WS_ID="${WS_ID:-ws-rt003-${RUN_ID}}"
TRACE_ID="${TRACE_ID:-trace-rt003-${RUN_ID}}"
RT_ID="${RT_ID:-rt003}"

TARGET_PROFILE="${TARGET_PROFILE:-local}"  # local|remote
REMOTE_DOMAIN="${REMOTE_DOMAIN:-example.invalid}"
TARGET_SCHEME="${TARGET_SCHEME:-$([[ "$TARGET_PROFILE" == "remote" ]] && echo https || echo http)}"
TARGET_HOST="${TARGET_HOST:-$([[ "$TARGET_PROFILE" == "remote" ]] && echo s3.${REMOTE_DOMAIN} || echo 127.0.0.1)}"
TARGET_PORT="${TARGET_PORT:-$([[ "$TARGET_PROFILE" == "remote" ]] && echo 443 || echo 8443)}"
TARGET_PATH="${TARGET_PATH:-/bucket/object}"
TARGET_URL="${TARGET_SCHEME}://${TARGET_HOST}:${TARGET_PORT}${TARGET_PATH}"

ROOT_SOCK="$HOME/.yai/run/root/root.sock"
ENGINE_SOCK="$HOME/.yai/run/engine/control.sock"
ROOT_LOG="$HOME/.yai/run/root/root.log"
ENGINE_LOG="$STATE_DIR/engine.log"
ROOT_STDERR_LOG="$STATE_DIR/root.stderr.log"
ROOT_STDOUT_LOG="$STATE_DIR/root.stdout.log"

YAI_ROOT_BIN="$REPO_ROOT/build/bin/yai-root-server"
YAI_ENGINE_BIN="$REPO_ROOT/build/bin/yai-engine"

mkdir -p "$EVIDENCE_DIR" "$STATE_DIR"

if [[ ! -f "$BASELINE_FILE" ]]; then
  echo "missing baseline file: $BASELINE_FILE" >&2
  exit 1
fi

wait_for_socket() {
  local sock="$1"
  local timeout_s="${2:-20}"
  local i
  for ((i=0; i<timeout_s*10; i++)); do
    [[ -S "$sock" ]] && return 0
    sleep 0.1
  done
  return 1
}

wait_for_pid_alive() {
  local pid="$1"
  local timeout_s="${2:-10}"
  local i
  for ((i=0; i<timeout_s*10; i++)); do
    kill -0 "$pid" >/dev/null 2>&1 && return 0
    sleep 0.1
  done
  return 1
}

export RT_DIR REPO_ROOT DOMAIN_PACK_ID BASELINE_ID RUN_ID WORKLOAD_ID ATTACK_PROFILE_ID
export PACK_DIR BASELINE_FILE EXPECTED_FILE EVIDENCE_DIR STATE_DIR
export WS_ID TRACE_ID RT_ID TARGET_PROFILE REMOTE_DOMAIN TARGET_SCHEME TARGET_HOST TARGET_PORT TARGET_PATH TARGET_URL ROOT_SOCK ENGINE_SOCK ROOT_LOG ENGINE_LOG ROOT_STDERR_LOG ROOT_STDOUT_LOG
export YAI_ROOT_BIN YAI_ENGINE_BIN
RT_ID="${RT_ID:-rt003}"
