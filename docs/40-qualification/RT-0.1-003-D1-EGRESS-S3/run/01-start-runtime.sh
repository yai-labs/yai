#!/usr/bin/env bash
set -euo pipefail
source "$(dirname "$0")/_lib.sh"

mkdir -p "$HOME/.yai/run/root" "$HOME/.yai/run/engine"
rm -f "$ROOT_SOCK" "$ENGINE_SOCK"


python3 - <<'PY'
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
        err = ctypes.get_errno()
        raise OSError(err, f"shm_open failed for {name}")
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
PY


if [[ ! -x "$YAI_ROOT_BIN" || ! -x "$YAI_ENGINE_BIN" ]]; then
  make all >/dev/null
fi

YAI_ENGINE_ALLOW_DEGRADED="1" YAI_EGRESS_ALLOWLIST="" YAI_PROVIDER_HOST="127.0.0.1" YAI_PROVIDER_PORT="8443" \
"$YAI_ENGINE_BIN" "$WS_ID" >"$ENGINE_LOG" 2>&1 &
ENGINE_PID=$!

wait_for_pid_alive "$ENGINE_PID" 10 || { echo "engine failed" >&2; exit 1; }
if [[ ! -S "$ENGINE_SOCK" ]]; then
  echo "engine control socket not exposed; continuing with root-governed path" >&2
fi

"$YAI_ROOT_BIN" >"$ROOT_STDOUT_LOG" 2>"$ROOT_STDERR_LOG" &
ROOT_PID=$!

wait_for_pid_alive "$ROOT_PID" 10 || { echo "root failed" >&2; exit 1; }
wait_for_socket "$ROOT_SOCK" 20 || { echo "root socket not ready" >&2; exit 1; }

ENGINE_PID="$ENGINE_PID" ROOT_PID="$ROOT_PID" python3 - <<'PY'
import datetime, json, os
state = {
  "mode": "live",
  "started_at": datetime.datetime.now(datetime.UTC).isoformat(),
  "run_id": os.environ["RUN_ID"],
  "ws_id": os.environ["WS_ID"],
  "trace_id": os.environ["TRACE_ID"],
  "sockets": {"root": os.environ["ROOT_SOCK"], "engine": os.environ["ENGINE_SOCK"]},
}
open(os.path.join(os.environ["STATE_DIR"], "runtime.json"), "w", encoding="utf-8").write(json.dumps(state, indent=2))
open(os.path.join(os.environ["STATE_DIR"], "pids.json"), "w", encoding="utf-8").write(json.dumps({
  "engine_pid": int(os.environ["ENGINE_PID"]),
  "root_pid": int(os.environ["ROOT_PID"]),
}, indent=2))
PY

echo "runtime started (live): $RUN_ID"
