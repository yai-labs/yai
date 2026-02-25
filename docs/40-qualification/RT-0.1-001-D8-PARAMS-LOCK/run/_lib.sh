#!/usr/bin/env bash
set -euo pipefail

RT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
REPO_ROOT="$(cd "$RT_DIR/../../.." && pwd)"

DOMAIN_PACK_ID="${DOMAIN_PACK_ID:-D8-scientific/reproducibility-parameter-lock-v1}"
BASELINE_ID="${BASELINE_ID:-baseline-deny}"
RUN_ID="${RUN_ID:-run-001}"
WORKLOAD_ID="${WORKLOAD_ID:-wrk-d8-params-lock-v1}"
ATTACK_PROFILE_ID="${ATTACK_PROFILE_ID:-rt-001-params-lock-attempt}"

PACK_DIR="$REPO_ROOT/docs/30-catalog/domains/packs/$DOMAIN_PACK_ID"
BASELINE_FILE="$PACK_DIR/contracts/${BASELINE_ID}.json"
EXPECTED_FILE="$PACK_DIR/vectors/expected_outcomes.json"
EVIDENCE_ROOT="${EVIDENCE_ROOT:-$RT_DIR/evidence.local}"
EVIDENCE_DIR="$EVIDENCE_ROOT/$DOMAIN_PACK_ID/$RUN_ID"
STATE_DIR="$RT_DIR/run/.state/$RUN_ID"

WS_SUFFIX="${WS_SUFFIX:-${RUN_ID##*-}}"
WS_ID="${WS_ID:-wrt8-${WS_SUFFIX}}"
TRACE_ID="${TRACE_ID:-trt8-${WS_SUFFIX}}"
RT_ID="${RT_ID:-rt001d8}"

TARGET_PROFILE="${TARGET_PROFILE:-local}" # local|docker
SHARED_DOCKER_DIR="$REPO_ROOT/docs/40-qualification/_shared/d8-artifact-store"
DOCKER_COMPOSE_FILE="$SHARED_DOCKER_DIR/docker-compose.yml"
DOCKER_STORE_DIR="$SHARED_DOCKER_DIR/_artifact_store"
LOCAL_STORE_DIR="$RT_DIR/target/_artifact_store"

if [[ "$TARGET_PROFILE" == "docker" ]]; then
  TARGET_STORE_DIR="$DOCKER_STORE_DIR"
  TARGET_BASE_URL="${TARGET_BASE_URL:-http://127.0.0.1:18080}"
else
  TARGET_STORE_DIR="$LOCAL_STORE_DIR"
  TARGET_BASE_URL="${TARGET_BASE_URL:-file://$LOCAL_STORE_DIR}"
fi

TARGET_DST_PATH="${TARGET_DST_PATH:-/runs/${RUN_ID}/result.json}"
TARGET_URL="${TARGET_BASE_URL}${TARGET_DST_PATH}"

PIPELINE_ID="${PIPELINE_ID:-PIPE-EXP-001}"
DATASET_REF="${DATASET_REF:-DATASET-TEST-001@sha256:0000000000000000000000000000000000000000000000000000000000000000}"

ROOT_SOCK="$HOME/.yai/run/root/root.sock"
KERNEL_SOCK="$HOME/.yai/run/kernel/control.sock"
ENGINE_SOCK="$HOME/.yai/run/engine/control.sock"
ROOT_LOG="$HOME/.yai/run/root/root.log"
KERNEL_LOG="$HOME/.yai/run/kernel/kernel.log"
BOOT_LOG="$STATE_DIR/boot.log"
ENGINE_LOG="$STATE_DIR/engine.log"
ROOT_STDERR_LOG="$STATE_DIR/root.stderr.log"
ROOT_STDOUT_LOG="$STATE_DIR/root.stdout.log"

YAI_BOOT_BIN="$REPO_ROOT/build/bin/yai-boot"
YAI_ROOT_BIN="$REPO_ROOT/build/bin/yai-root-server"
YAI_ENGINE_BIN="$REPO_ROOT/build/bin/yai-engine"

YAI_CLI_REPO="$REPO_ROOT/../yai-cli"
YAI_BIN_DEFAULT="$(command -v yai || true)"
YAI_BIN_ALT="$YAI_CLI_REPO/dist/bin/yai"
YAI_BIN_ALT_LEGACY="$YAI_CLI_REPO/dist/bin/yai-cli"

if [[ -n "${YAI_BIN:-}" ]]; then
  :
elif [[ -n "$YAI_BIN_DEFAULT" ]] && "$YAI_BIN_DEFAULT" up --help >/dev/null 2>&1; then
  YAI_BIN="$YAI_BIN_DEFAULT"
elif [[ -x "$YAI_BIN_ALT" ]]; then
  YAI_BIN="$YAI_BIN_ALT"
elif [[ -x "$YAI_BIN_ALT_LEGACY" ]]; then
  YAI_BIN="$YAI_BIN_ALT_LEGACY"
else
  YAI_BIN="$YAI_BIN_DEFAULT"
fi

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
export PACK_DIR BASELINE_FILE EXPECTED_FILE EVIDENCE_ROOT EVIDENCE_DIR STATE_DIR
export WS_ID TRACE_ID RT_ID TARGET_PROFILE SHARED_DOCKER_DIR DOCKER_COMPOSE_FILE DOCKER_STORE_DIR LOCAL_STORE_DIR TARGET_STORE_DIR TARGET_BASE_URL TARGET_DST_PATH TARGET_URL PIPELINE_ID DATASET_REF
export ROOT_SOCK KERNEL_SOCK ENGINE_SOCK ROOT_LOG KERNEL_LOG BOOT_LOG ENGINE_LOG ROOT_STDERR_LOG ROOT_STDOUT_LOG
export YAI_BOOT_BIN YAI_ROOT_BIN YAI_ENGINE_BIN YAI_BIN

wait_for_root_cli_ready() {
  local timeout_s="${1:-20}"
  local i
  for ((i=0; i<timeout_s*2; i++)); do
    if [[ -n "$YAI_BIN" ]] && "$YAI_BIN" root ping >/dev/null 2>&1; then
      return 0
    fi
    sleep 0.5
  done
  return 1
}
