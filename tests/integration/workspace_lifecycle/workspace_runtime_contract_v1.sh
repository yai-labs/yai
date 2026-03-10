#!/usr/bin/env bash
set -euo pipefail

REPO="$(cd "$(dirname "${BASH_SOURCE[0]}")/../../.." && pwd)"
YAI="$REPO/build/bin/yai"
SOCK="${YAI_RUNTIME_INGRESS:-$HOME/.yai/run/control.sock}"

if [[ ! -x "$YAI" ]]; then
  make -C "$REPO" yai >/dev/null
fi

"$YAI" down >/dev/null 2>&1 || true
rm -f "$SOCK" >/dev/null 2>&1 || true

RUNTIME_PID=""
cleanup() {
  if [[ -n "$RUNTIME_PID" ]] && kill -0 "$RUNTIME_PID" 2>/dev/null; then
    kill "$RUNTIME_PID" >/dev/null 2>&1 || true
    wait "$RUNTIME_PID" >/dev/null 2>&1 || true
  fi
}
trap cleanup EXIT

(cd "$REPO" && "$YAI" >/tmp/yai_workspace_runtime_up.log 2>&1) &
RUNTIME_PID=$!

for _ in $(seq 1 50); do
  if [[ -S "$SOCK" ]]; then
    break
  fi
  sleep 0.1
done

if [[ ! -S "$SOCK" ]]; then
  echo "workspace_runtime_contract_v1: FAIL (missing ingress socket $SOCK)"
  exit 1
fi

python3 - <<'PY'
import socket
import struct
import json
import os

SOCK = os.environ.get("YAI_RUNTIME_INGRESS", os.path.expanduser("~/.yai/run/control.sock"))
YAI_FRAME_MAGIC = 0x59414950
YAI_PROTOCOL_IDS_VERSION = 1
YAI_CMD_HANDSHAKE = 0x0102
YAI_CMD_CONTROL_CALL = 0x0105
ENV_FMT = "<II36s36sIHBBII"
REQ_FMT = "<II32s"


def build(cmd_id, ws_id, payload, trace):
    ws = ws_id.encode("utf-8")[:36].ljust(36, b"\0")
    tr = trace.encode("utf-8")[:36].ljust(36, b"\0")
    env = struct.pack(ENV_FMT, YAI_FRAME_MAGIC, YAI_PROTOCOL_IDS_VERSION, ws, tr, cmd_id, 2, 1, 0, len(payload), 0)
    return env + payload


def recv_exact(sock, n):
    out = b""
    while len(out) < n:
      c = sock.recv(n - len(out))
      if not c:
        raise RuntimeError("eof")
      out += c
    return out


def call(ws_id):
    s = socket.socket(socket.AF_UNIX, socket.SOCK_STREAM)
    s.connect(SOCK)
    hs_payload = struct.pack(REQ_FMT, YAI_PROTOCOL_IDS_VERSION, 0, b"yai-test")
    s.sendall(build(YAI_CMD_HANDSHAKE, ws_id, hs_payload, "hs-1"))
    env = recv_exact(s, 96)
    _, _, _, _, cmd, _, _, _, plen, _ = struct.unpack(ENV_FMT, env)
    if cmd != YAI_CMD_HANDSHAKE:
      raise RuntimeError("bad handshake response")
    recv_exact(s, plen)

    payload = json.dumps({"type":"yai.control.call.v1","command_id":"yai.runtime.ping","target_plane":"runtime","argv":["ping"]}).encode("utf-8")
    s.sendall(build(YAI_CMD_CONTROL_CALL, ws_id, payload, "call-1"))
    env = recv_exact(s, 96)
    _, _, _, _, cmd, _, _, _, plen, _ = struct.unpack(ENV_FMT, env)
    if cmd != YAI_CMD_CONTROL_CALL:
      raise RuntimeError("bad control response")
    body = recv_exact(s, plen).decode("utf-8")
    s.close()
    obj = json.loads(body)
    if obj.get("status") != "ok" or obj.get("code") not in ("OK", "REVIEW_REQUIRED"):
      raise RuntimeError(f"unexpected reply {obj}")

for ws in ("ws_contract_01", "ws_contract_02", "ws_contract_ops"):
    call(ws)
PY

for ws in ws_contract_01 ws_contract_02 ws_contract_ops; do
  if [[ ! -d "$HOME/.yai/run/$ws" ]]; then
    echo "workspace_runtime_contract_v1: FAIL (missing run dir for $ws)"
    exit 1
  fi
done

echo "workspace_runtime_contract_v1: ok"
