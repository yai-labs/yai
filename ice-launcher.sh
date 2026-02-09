#!/bin/bash

# --- CONFIGURAZIONE ---
WS_ID="arch_dev_session"
WITH_ENGINE=1
WITH_API=1
WITH_ORCH=0
TASK=""
TASK_TEXT=""
SHM_NAME="/ice_vault_$WS_ID"
SOCK_PATH="/tmp/ice_runtime.sock"
LOG_PATH=""
VAULTS=("core" "stream" "brain" "audit" "cache" "control")
SHM_BASE="/ice_vault_$WS_ID"
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
ROOT_DIR="${SCRIPT_DIR}"
ICE_API_HOST="127.0.0.1"
ICE_API_PORT="8081"
ICE_API_BIN="${ROOT_DIR}/../Api/target/release/ice-api"
ICE_API_VENV="${ROOT_DIR}/../Api/.venv"
ICE_REMOTE_ENDPOINT="http://192.168.0.98:8080/v1/chat/completions"
ICE_REMOTE_MODEL="qwen2.5-coder-7b-instruct-q4_k_m"
ICE_AI_AGENTS="system,code-fs,code-llm,git,historian,knowledge,validator"
ICE_CONTEXT_FILES="${ROOT_DIR}/../ice-ai/FOUNDATION.md,${ROOT_DIR}/../ice-ai/GOVERNANCE.md"
ICE_ENGINE_BIN="${ROOT_DIR}/../Engine/ice-engine"
ICE_KNOWLEDGE_DB="${ROOT_DIR}/../Consciousness/storage/knowledge.db"
ICE_LOG_FILE="${ROOT_DIR}/../ice_runtime.log"

OS_NAME="$(uname -s)"
if [ "$OS_NAME" = "Linux" ]; then
    SHM_PATH="/dev/shm/ice_vault_$WS_ID"
else
    SHM_PATH=""
fi

usage() {
    echo "Usage: ./ice-launcher.sh [--ws <id>] [--no-engine] [--no-api] [--orch] [--smoke]"
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
            SHM_NAME="/ice_vault_$WS_ID"
            if [ "$OS_NAME" = "Linux" ]; then
                SHM_PATH="/dev/shm/ice_vault_$WS_ID"
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

# --- ICE PROTOCOL ALPHABET ---
SIG_OK="[$(echo -e '\033[0;32mOK\033[0m')]"
SIG_INF="[$(echo -e '\033[0;34m..\033[0m')]"
SIG_ERR="[$(echo -e '\033[0;31m!!\033[0m')]"
SIG_WRN="[$(echo -e '\033[0;33m??\033[0m')]"

ice_log() {
    local layer=$1
    local action=$2
    local signal=$3
    local msg=$4
    echo -e "${signal} [${layer}:${action}] ${msg}"
}

: > "$ICE_LOG_FILE"
ice_log "SYS" "BOOT" "$SIG_INF" "Starting Full Stack Orchestration (WS: $WS_ID)"

# 1. Pulizia preventiva (RAID)
ice_log "VLT" "SYNC" "$SIG_INF" "Sanitizing memory channels..."
if [ "$OS_NAME" = "Linux" ]; then
    for v in "${VAULTS[@]}"; do
        rm -f "/dev/shm${SHM_BASE}_$v" 2>/dev/null
    done
fi
rm -f $SOCK_PATH

cleanup() {
    echo ""
    ice_log "SYS" "HALT" "$SIG_INF" "Shutting down all subsystems..."
    kill $(jobs -p) 2>/dev/null
    rm -f $SOCK_PATH
    exit
}

trap cleanup SIGINT SIGTERM

# 2. AVVIO BOOTSTRAP (crea SHM)
ice_log "SYS" "BOOTSTRAP" "$SIG_INF" "Launching BOOTSTRAP..."
if [ ! -x "${ROOT_DIR}/bin/ice-boot" ]; then
    ice_log "SYS" "BUILD" "$SIG_INF" "Building BOOTSTRAP..."
    (cd "${ROOT_DIR}" && make boot) >> "$ICE_LOG_FILE" 2>&1
fi

(cd "${ROOT_DIR}" && ./bin/ice-boot --ws $WS_ID --raid) >> "$ICE_LOG_FILE" 2>&1 &
sleep 1

if [ $? -ne 0 ]; then
    ice_log "VLT" "CORE" "$SIG_ERR" "Strap failed to create SHM at $SHM_NAME"
    cleanup
fi
if [ -n "$SHM_PATH" ] && [ ! -f "$SHM_PATH" ]; then
    ice_log "VLT" "CORE" "$SIG_ERR" "Strap failed to create SHM at $SHM_PATH"
    cleanup
fi

# VERIFICA RAID (solo Linux)
ice_log "VLT" "RAID" "$SIG_INF" "Inspecting cognitive RAID..."
if [ "$OS_NAME" = "Linux" ]; then
    for v in "${VAULTS[@]}"; do
        if [ "$v" = "core" ]; then
            TARGET="/dev/shm${SHM_BASE}"
        else
            TARGET="/dev/shm${SHM_BASE}_$v"
        fi
        if [ -f "$TARGET" ]; then
            SIZE=$(stat -c%s "$TARGET")
            ice_log "VLT" "RAID" "$SIG_OK" "Channel $v ONLINE (${SIZE} bytes)"
        else
            ice_log "VLT" "RAID" "$SIG_ERR" "Channel $v OFFLINE"
            cleanup
        fi
    done
fi

run_engine() {
    (cd "${ROOT_DIR}/.." && "${ICE_ENGINE_BIN}" $WS_ID 2>&1 | tee -a "$LOG_PATH")
}

run_api() {
    local host="$ICE_API_HOST"
    local port="$ICE_API_PORT"
    local log_path="${ROOT_DIR}/../api_debug.log"
    ice_log "API" "LIST" "$SIG_INF" "Launching ICE-API (ws://${host}:${port})..."
    : > "$log_path"

    if [ -x "${ICE_API_BIN}" ]; then
        ice_log "API" "BIN" "$SIG_OK" "Rust: ${ICE_API_BIN}"
        ICE_API_HOST="$host" ICE_API_PORT="$port" \
          ICE_REMOTE_ENDPOINT="${ICE_REMOTE_ENDPOINT}" \
          ICE_REMOTE_MODEL="${ICE_REMOTE_MODEL}" \
          ICE_AI_AGENTS="${ICE_AI_AGENTS}" \
          ICE_CONTEXT_FILES="${ICE_CONTEXT_FILES}" \
          ICE_ENGINE_BIN="${ICE_ENGINE_BIN}" \
          ICE_WORKSPACE_ROOT="${ROOT_DIR}/.." \
          ICE_WORKSPACE_ID="${WS_ID}" \
          ICE_KNOWLEDGE_DB="${ICE_KNOWLEDGE_DB}" \
          "${ICE_API_BIN}" 2>&1 | tee -a "$log_path" &
        sleep 1
        return
    fi

    if [ -f "${ROOT_DIR}/../Api/Cargo.toml" ]; then
        ice_log "API" "BUILD" "$SIG_INF" "Building ICE-API (Rust)..."
        (cd "${ROOT_DIR}/../Api" && cargo build --release) >> "$log_path" 2>&1
        if [ -x "${ICE_API_BIN}" ]; then
            ice_log "API" "BIN" "$SIG_OK" "Rust: ${ICE_API_BIN}"
            ICE_API_HOST="$host" ICE_API_PORT="$port" \
              ICE_REMOTE_ENDPOINT="${ICE_REMOTE_ENDPOINT}" \
              ICE_REMOTE_MODEL="${ICE_REMOTE_MODEL}" \
              ICE_AI_AGENTS="${ICE_AI_AGENTS}" \
              ICE_CONTEXT_FILES="${ICE_CONTEXT_FILES}" \
              ICE_ENGINE_BIN="${ICE_ENGINE_BIN}" \
              ICE_WORKSPACE_ROOT="${ROOT_DIR}/.." \
              ICE_WORKSPACE_ID="${WS_ID}" \
              ICE_KNOWLEDGE_DB="${ICE_KNOWLEDGE_DB}" \
              "${ICE_API_BIN}" 2>&1 | tee -a "$log_path" &
            sleep 1
            return
        fi
    fi

    ice_log "API" "ERR" "$SIG_ERR" "ICE-API binary not found."
    sleep 1
}

run_orchestrator() {
    ice_log "API" "SMOKE" "$SIG_INF" "Running orchestrator PING->PONG..."
    (cd "${ROOT_DIR}/../Api" && ICE_WORKSPACE_ID="$WS_ID" cargo run --bin orchestrator -p ice-api) 2>&1 | tee -a "$ICE_LOG_FILE"
}

# AVVIO API
if [ "$WITH_API" -eq 1 ]; then
    run_api
else
    ice_log "API" "SKIP" "$SIG_WRN" "ICE-API skipped (--no-api)"
fi

# AVVIO ENGINE
if [ "$WITH_ENGINE" -eq 1 ]; then
    ice_log "ENG" "INIT" "$SIG_INF" "Launching ENGINE..."
    if [ ! -x "${ICE_ENGINE_BIN}" ]; then
        ice_log "ENG" "BUILD" "$SIG_INF" "Building ENGINE..."
        (cd "${ROOT_DIR}/../Engine" && make clean && make) >> "$ICE_LOG_FILE" 2>&1
    fi
    LOG_PATH="${ROOT_DIR}/../engine_debug.log"
    run_engine
else
    ice_log "ENG" "SKIP" "$SIG_WRN" "ENGINE skipped (--no-engine)"
fi

if [ "$WITH_ORCH" -eq 1 ]; then
    run_orchestrator
fi

cleanup
