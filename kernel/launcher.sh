#!/bin/bash

# --- CONFIGURAZIONE ---
WS_ID="arch_dev_session"
WITH_ENGINE=1
WITH_API=1
WITH_ORCH=0
TASK=""
TASK_TEXT=""
SHM_NAME="/yai_vault_$WS_ID"
SOCK_PATH="/tmp/yai_runtime.sock"
LOG_PATH=""
VAULTS=("core" "stream" "brain" "audit" "cache" "control")
SHM_BASE="/yai_vault_$WS_ID"
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
ROOT_DIR="${SCRIPT_DIR}"
YAI_WORKSPACE_ROOT="${ROOT_DIR}/.."
YAI_HOME="${YAI_WORKSPACE_ROOT}/.yai"
mkdir -p "${YAI_HOME}/logs"

YAI_API_HOST="127.0.0.1"
YAI_API_PORT="8081"
YAI_API_BIN="${YAI_HOME}/artifacts/yai-mind/target/release/yai-mind"
YAI_REMOTE_ENDPOINT="http://192.168.0.98:8080/v1/chat/completions"
YAI_REMOTE_MODEL="qwen2.5-coder-7b-instruct-q4_k_m"
YAI_AI_AGENTS="system,code-fs,code-llm,git,historian,knowledge,validator"
YAI_CONTEXT_FILES="${ROOT_DIR}/../yai-ai/FOUNDATION.md,${ROOT_DIR}/../yai-ai/GOVERNANCE.md"
YAI_ENGINE_BIN="${YAI_HOME}/artifacts/yai-engine/bin/yai-engine"
YAI_KNOWLEDGE_DB="${YAI_HOME}/data/db/knowledge.db"
YAI_LOG_FILE="${YAI_HOME}/logs/yai_runtime.log"

OS_NAME="$(uname -s)"
if [ "$OS_NAME" = "Linux" ]; then
    SHM_PATH="/dev/shm/yai_vault_$WS_ID"
else
    SHM_PATH=""
fi

usage() {
    echo "Usage: ./launcher.sh [--ws <id>] [--no-engine] [--no-api] [--orch] [--smoke]"
    echo "       --orch  lancia orchestrator (PING->PONG) dopo Engine"
    echo "       --smoke equivale a --no-api --orch"
}

while [ $# -gt 0 ]; do
    case "$1" in
        --ws)
            if [ -z "$2" ]; then
                usage
                exit 1
            fi
            WS_ID="$2"
            SHM_NAME="/yai_vault_$WS_ID"
            if [ "$OS_NAME" = "Linux" ]; then
                SHM_PATH="/dev/shm/yai_vault_$WS_ID"
            else
                SHM_PATH=""
            fi
            shift 2
            ;;
        --no-engine)
            WITH_ENGINE=0
            shift
            ;;
        --no-api)
            WITH_API=0
            shift
            ;;
        --orch)
            WITH_ORCH=1
            shift
            ;;
        --smoke)
            WITH_API=0
            WITH_ORCH=1
            shift
            ;;
        -h|--help)
            usage
            exit 0
            ;;
        *)
            echo "Unknown option: $1"
            usage
            exit 1
            ;;
    esac
 done

# Colori per i log
RED='\033[0;31m'
GREEN='\033[0;32m'
BLUE='\033[0;34m'
YELLOW='\033[0;33m'
NC='\033[0m'

# --- YAI PROTOCOL ALPHABET ---
SIG_OK="[$(echo -e '\033[0;32mOK\033[0m')]"
SIG_INF="[$(echo -e '\033[0;34m..\033[0m')]"
SIG_ERR="[$(echo -e '\033[0;31m!!\033[0m')]"
SIG_WRN="[$(echo -e '\033[0;33m??\033[0m')]"

yai_log() {
    local layer=$1
    local action=$2
    local signal=$3
    local msg=$4
    echo -e "${signal} [${layer}:${action}] ${msg}"
}

: > "$YAI_LOG_FILE"
yai_log "SYS" "BOOT" "$SIG_INF" "Starting Full Stack Orchestration (WS: $WS_ID)"

# 1. Pulizia preventiva (RAID)
yai_log "VLT" "SYNC" "$SIG_INF" "Sanitizing memory channels..."
if [ "$OS_NAME" = "Linux" ]; then
    for v in "${VAULTS[@]}"; do
        rm -f "/dev/shm${SHM_BASE}_$v" 2>/dev/null
    done
fi
rm -f $SOCK_PATH

cleanup() {
    echo ""
    yai_log "SYS" "HALT" "$SIG_INF" "Shutting down all subsystems..."
    kill $(jobs -p) 2>/dev/null
    rm -f $SOCK_PATH
    exit
}

trap cleanup SIGINT SIGTERM

# 2. AVVIO BOOTSTRAP (crea SHM)
yai_log "SYS" "BOOTSTRAP" "$SIG_INF" "Launching BOOTSTRAP..."
if [ ! -x "${YAI_HOME}/artifacts/yai-kernel/bin/yai-boot" ]; then
    yai_log "SYS" "BUILD" "$SIG_INF" "Building BOOTSTRAP..."
    (cd "${ROOT_DIR}" && make boot) >> "$YAI_LOG_FILE" 2>&1
fi

(cd "${ROOT_DIR}" && "${YAI_HOME}/artifacts/yai-kernel/bin/yai-boot" --ws $WS_ID --raid) >> "$YAI_LOG_FILE" 2>&1 &
sleep 1

if [ $? -ne 0 ]; then
    yai_log "VLT" "CORE" "$SIG_ERR" "Strap failed to create SHM at $SHM_NAME"
    cleanup
fi
if [ -n "$SHM_PATH" ] && [ ! -f "$SHM_PATH" ]; then
    yai_log "VLT" "CORE" "$SIG_ERR" "Strap failed to create SHM at $SHM_PATH"
    cleanup
fi

# VERIFICA RAID (solo Linux)
yai_log "VLT" "RAID" "$SIG_INF" "Inspecting cognitive RAID..."
if [ "$OS_NAME" = "Linux" ]; then
    for v in "${VAULTS[@]}"; do
        if [ "$v" = "core" ]; then
            TARGET="/dev/shm${SHM_BASE}"
        else
            TARGET="/dev/shm${SHM_BASE}_$v"
        fi
        if [ -f "$TARGET" ]; then
            SIZE=$(stat -c%s "$TARGET")
            yai_log "VLT" "RAID" "$SIG_OK" "Channel $v ONLINE (${SIZE} bytes)"
        else
            yai_log "VLT" "RAID" "$SIG_ERR" "Channel $v OFFLINE"
            cleanup
        fi
    done
fi

run_engine() {
    (cd "${YAI_WORKSPACE_ROOT}" && "${YAI_ENGINE_BIN}" $WS_ID 2>&1 | tee -a "$LOG_PATH")
}

run_api() {
    local host="$YAI_API_HOST"
    local port="$YAI_API_PORT"
    local log_path="${YAI_HOME}/logs/api_debug.log"
    yai_log "API" "LIST" "$SIG_INF" "Launching YAI-MIND (ws://${host}:${port})..."
    : > "$log_path"

    if [ -x "${YAI_API_BIN}" ]; then
        yai_log "API" "BIN" "$SIG_OK" "Rust: ${YAI_API_BIN}"
        YAI_API_HOST="$host" YAI_API_PORT="$port" \
          YAI_REMOTE_ENDPOINT="${YAI_REMOTE_ENDPOINT}" \
          YAI_REMOTE_MODEL="${YAI_REMOTE_MODEL}" \
          YAI_AI_AGENTS="${YAI_AI_AGENTS}" \
          YAI_CONTEXT_FILES="${YAI_CONTEXT_FILES}" \
          YAI_ENGINE_BIN="${YAI_ENGINE_BIN}" \
          YAI_WORKSPACE_ROOT="${YAI_WORKSPACE_ROOT}" \
          YAI_WORKSPACE_ID="${WS_ID}" \
          YAI_KNOWLEDGE_DB="${YAI_KNOWLEDGE_DB}" \
          "${YAI_API_BIN}" 2>&1 | tee -a "$log_path" &
        sleep 1
        return
    fi

    if [ -f "${ROOT_DIR}/../yai-mind/Cargo.toml" ]; then
        yai_log "API" "BUILD" "$SIG_INF" "Building YAI-MIND (Rust)..."
        (cd "${ROOT_DIR}/../yai-mind" && cargo build --release) >> "$log_path" 2>&1
        if [ -x "${YAI_API_BIN}" ]; then
            yai_log "API" "BIN" "$SIG_OK" "Rust: ${YAI_API_BIN}"
            YAI_API_HOST="$host" YAI_API_PORT="$port" \
              YAI_REMOTE_ENDPOINT="${YAI_REMOTE_ENDPOINT}" \
              YAI_REMOTE_MODEL="${YAI_REMOTE_MODEL}" \
              YAI_AI_AGENTS="${YAI_AI_AGENTS}" \
              YAI_CONTEXT_FILES="${YAI_CONTEXT_FILES}" \
              YAI_ENGINE_BIN="${YAI_ENGINE_BIN}" \
              YAI_WORKSPACE_ROOT="${YAI_WORKSPACE_ROOT}" \
              YAI_WORKSPACE_ID="${WS_ID}" \
              YAI_KNOWLEDGE_DB="${YAI_KNOWLEDGE_DB}" \
              "${YAI_API_BIN}" 2>&1 | tee -a "$log_path" &
            sleep 1
            return
        fi
    fi

    yai_log "API" "ERR" "$SIG_ERR" "YAI-MIND binary not found."
    sleep 1
}

run_orchestrator() {
    yai_log "API" "SMOKE" "$SIG_INF" "Running orchestrator PING->PONG..."
    (cd "${ROOT_DIR}/../yai-mind" && YAI_WORKSPACE_ID="$WS_ID" cargo run --bin yai-orchestrator -p yai-mind) 2>&1 | tee -a "$YAI_LOG_FILE"
}

# AVVIO API
if [ "$WITH_API" -eq 1 ]; then
    run_api
else
    yai_log "API" "SKIP" "$SIG_WRN" "YAI-MIND skipped (--no-api)"
fi

# AVVIO ENGINE
if [ "$WITH_ENGINE" -eq 1 ]; then
    yai_log "ENG" "INIT" "$SIG_INF" "Launching ENGINE..."
    if [ ! -x "${YAI_ENGINE_BIN}" ]; then
        yai_log "ENG" "BUILD" "$SIG_INF" "Building ENGINE..."
        (cd "${ROOT_DIR}/../yai-engine" && make clean && make) >> "$YAI_LOG_FILE" 2>&1
    fi
    LOG_PATH="${YAI_HOME}/logs/engine_debug.log"
    run_engine
else
    yai_log "ENG" "SKIP" "$SIG_WRN" "ENGINE skipped (--no-engine)"
fi

if [ "$WITH_ORCH" -eq 1 ]; then
    run_orchestrator
fi

cleanup
