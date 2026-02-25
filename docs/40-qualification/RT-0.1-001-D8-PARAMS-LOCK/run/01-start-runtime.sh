#!/usr/bin/env bash
set -euo pipefail
source "$(dirname "$0")/_lib.sh"

mkdir -p "$HOME/.yai/run/root" "$HOME/.yai/run/kernel" "$HOME/.yai/run/engine"
rm -f "$ROOT_SOCK" "$KERNEL_SOCK" "$ENGINE_SOCK"

python3 - <<'PY2'
import ctypes, os

ws = os.environ["WS_ID"]
libc = ctypes.CDLL(None, use_errno=True)
O_CREAT = os.O_CREAT
O_RDWR = os.O_RDWR
PROT_READ = 0x1
PROT_WRITE = 0x2
MAP_SHARED = 0x01
SIZE = 4096

libc.shm_open.argtypes = [ctypes.c_char_p, ctypes.c_int, ctypes.c_int]
libc.shm_open.restype = ctypes.c_int
libc.shm_unlink.argtypes = [ctypes.c_char_p]
libc.shm_unlink.restype = ctypes.c_int
libc.ftruncate.argtypes = [ctypes.c_int, ctypes.c_long]
libc.ftruncate.restype = ctypes.c_int
libc.mmap.argtypes = [ctypes.c_void_p, ctypes.c_size_t, ctypes.c_int, ctypes.c_int, ctypes.c_int, ctypes.c_long]
libc.mmap.restype = ctypes.c_void_p
libc.close.argtypes = [ctypes.c_int]
libc.close.restype = ctypes.c_int

def ensure(name: str):
    n = name.encode()
    libc.shm_unlink(n)
    fd = libc.shm_open(n, O_CREAT | O_RDWR, 0o666)
    if fd < 0:
        raise OSError(ctypes.get_errno(), f"shm_open failed for {name}")
    if libc.ftruncate(fd, SIZE) != 0:
        err = ctypes.get_errno()
        libc.close(fd)
        raise OSError(err, f"ftruncate failed for {name}")
    ptr = libc.mmap(None, SIZE, PROT_READ | PROT_WRITE, MAP_SHARED, fd, 0)
    if ptr in (ctypes.c_void_p(-1).value, None):
        err = ctypes.get_errno()
        libc.close(fd)
        raise OSError(err, f"mmap failed for {name}")
    buf = (ctypes.c_ubyte * SIZE).from_address(ptr)
    for i in range(0, 256):
        buf[i] = 0
    quota = 100000
    buf[4:8] = quota.to_bytes(4, "little")
    ws_b = ws.encode()[:63]
    for i, b in enumerate(ws_b):
        buf[12 + i] = b
    buf[12 + len(ws_b)] = 0
    buf[140] = 0
    libc.close(fd)

ensure(f"/yai_vault_{ws}")
ensure(f"/yai_vault_{ws}_CORE")
PY2

if [[ "$TARGET_PROFILE" == "docker" ]]; then
  mkdir -p "$DOCKER_STORE_DIR"
  docker compose -f "$DOCKER_COMPOSE_FILE" up -d >/dev/null
else
  mkdir -p "$LOCAL_STORE_DIR"
fi

if [[ ! -x "$YAI_BOOT_BIN" || ! -x "$YAI_ENGINE_BIN" ]]; then
  make all >/dev/null
fi

if [[ -z "$YAI_BIN" || ! -x "$YAI_BIN" ]]; then
  echo "yai CLI not found (set YAI_BIN or build yai-cli)" >&2
  exit 1
fi

YAI_BOOT_BIN="$YAI_BOOT_BIN" YAI_ENGINE_BIN="$YAI_ENGINE_BIN"   "$YAI_BIN" up --ws "$WS_ID" --detach --allow-degraded >>"$BOOT_LOG" 2>&1

wait_for_socket "$ROOT_SOCK" 20 || { echo "root socket not ready" >&2; exit 1; }
wait_for_socket "$KERNEL_SOCK" 20 || { echo "kernel socket not ready" >&2; exit 1; }
if ! wait_for_root_cli_ready 20; then
  echo "root CLI ping not ready" >&2
  exit 1
fi

BOOT_PID="$(pgrep -f yai-boot | tail -n1 || true)"
ROOT_PID="$(pgrep -f yai-root-server | tail -n1 || true)"
KERNEL_PID="$(pgrep -f yai-kernel | tail -n1 || true)"
ENGINE_PID="$(pgrep -f "yai-engine.*$WS_ID" | tail -n1 || true)"

if [[ -z "${ENGINE_PID:-}" ]] || ! kill -0 "${ENGINE_PID}" >/dev/null 2>&1; then
  echo "engine attach check failed: no live yai-engine process for ws=$WS_ID" >&2
  exit 1
fi

ENGINE_PROBE_ID="engine-attach-${RUN_ID}"
probe_ok=0
for _ in $(seq 1 40); do
  if "$YAI_BIN" engine --ws "$WS_ID" --arming --role operator storage put_node '{"id":"'"$ENGINE_PROBE_ID"'","kind":"attach_probe","meta":{"source":"sc102"}}' >/dev/null 2>&1 &&      "$YAI_BIN" engine --ws "$WS_ID" --arming --role operator storage get_node '{"id":"'"$ENGINE_PROBE_ID"'"}' >/dev/null 2>&1; then
    probe_ok=1
    break
  fi
  sleep 0.5
done
if [[ "$probe_ok" != "1" ]]; then
  echo "engine attach check failed: rpc probe did not succeed for ws=$WS_ID" >&2
  exit 1
fi
echo "engine attach verified: rpc_probe id=$ENGINE_PROBE_ID pid=$ENGINE_PID" >&2
export ENGINE_PID ENGINE_PROBE_ID

python3 - <<PY2
import json, os
open(os.path.join(os.environ["STATE_DIR"], "pids.json"), "w", encoding="utf-8").write(json.dumps({
  "boot_pid": int("${BOOT_PID:-0}" or 0),
  "root_pid": int("${ROOT_PID:-0}" or 0),
  "kernel_pid": int("${KERNEL_PID:-0}" or 0),
  "engine_pid": int("${ENGINE_PID:-0}" or 0),
}, indent=2))
PY2

if [[ ! -S "$ENGINE_SOCK" ]]; then
  echo "engine control socket not exposed; attach verified by live process pid=$ENGINE_PID" >&2
fi

python3 - <<'PY2'
import json, os, datetime
state = {
  "runtime_mode": "live",
  "topology": "boot->root->kernel->engine",
  "started_at": datetime.datetime.now(datetime.UTC).isoformat(),
  "domain_pack_id": os.environ["DOMAIN_PACK_ID"],
  "baseline_id": os.environ["BASELINE_ID"],
  "run_id": os.environ["RUN_ID"],
  "target_profile": os.environ["TARGET_PROFILE"],
  "sockets": {
    "root": os.environ["ROOT_SOCK"],
    "kernel": os.environ["KERNEL_SOCK"],
    "engine": os.environ["ENGINE_SOCK"],
  },
  "engine_attach": {
    "required": True,
    "engine_pid": int(os.environ.get("ENGINE_PID", "0") or 0),
    "probe_id": os.environ.get("ENGINE_PROBE_ID", ""),
    "method": "rpc_probe",
    "ok": int(os.environ.get("ENGINE_PID", "0") or 0) > 0,
  }
}
open(os.path.join(os.environ["STATE_DIR"], "runtime.json"), "w", encoding="utf-8").write(json.dumps(state, indent=2))
PY2

echo "runtime started (live): $RUN_ID (boot->root->kernel->engine)"
