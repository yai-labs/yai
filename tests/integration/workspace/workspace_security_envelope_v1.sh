#!/usr/bin/env bash
set -euo pipefail

REPO="$(cd "$(dirname "${BASH_SOURCE[0]}")/../../.." && pwd)"
YAI="$REPO/build/bin/yai"
SOCK="${YAI_RUNTIME_INGRESS:-$HOME/.yai/run/control.sock}"
WS="ws_env_v1"

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

(cd "$REPO" && "$YAI" >/tmp/yai_workspace_security_envelope_runtime.log 2>&1) &
RUNTIME_PID=$!

for _ in $(seq 1 50); do
  [[ -S "$SOCK" ]] && break
  sleep 0.1
done
[[ -S "$SOCK" ]] || { echo "workspace_security_envelope_v1: FAIL (missing ingress socket)"; exit 1; }

python3 - "$SOCK" "$WS" <<'PY'
import json
import socket
import struct
import sys

SOCK = sys.argv[1]
WS = sys.argv[2]
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

def call(ws_id, command_id, argv=None):
    if argv is None:
        argv = []
    s = socket.socket(socket.AF_UNIX, socket.SOCK_STREAM)
    s.connect(SOCK)

    hs_payload = struct.pack(REQ_FMT, YAI_PROTOCOL_IDS_VERSION, 0, b"yai-test")
    s.sendall(build(YAI_CMD_HANDSHAKE, ws_id, hs_payload, "hs"))
    env = recv_exact(s, 96)
    _, _, _, _, cmd, _, _, _, plen, _ = struct.unpack(ENV_FMT, env)
    if cmd != YAI_CMD_HANDSHAKE:
        raise RuntimeError("bad handshake")
    recv_exact(s, plen)

    payload = json.dumps({
        "type": "yai.control.call.v1",
        "command_id": command_id,
        "target_plane": "runtime",
        "argv": argv,
    }).encode("utf-8")

    s.sendall(build(YAI_CMD_CONTROL_CALL, ws_id, payload, "call"))
    env = recv_exact(s, 96)
    _, _, _, _, cmd, _, _, _, plen, _ = struct.unpack(ENV_FMT, env)
    if cmd != YAI_CMD_CONTROL_CALL:
        raise RuntimeError("bad control response")
    body = recv_exact(s, plen).decode("utf-8")
    s.close()
    return json.loads(body)

r = call(WS, "yai.workspace.create", [WS])
assert r["status"] == "ok", r
r = call("system", "yai.workspace.set", [WS])
assert r["status"] == "ok", r

s = call("system", "yai.workspace.status")
assert s["status"] == "ok", s
assert s["data"]["security_level_declared"] in ("scoped", "logical", "isolated", "sandboxed"), s
assert s["data"]["security_level_effective"] in ("scoped", "logical", "isolated", "sandboxed"), s
assert s["data"]["security_enforcement_mode"] != "", s
assert s["data"]["security_backend_mode"] != "", s
assert s["data"]["execution_mode_requested"] in ("scoped", "logical", "isolated", "sandboxed"), s
assert s["data"]["execution_mode_effective"] in ("scoped", "logical", "isolated", "sandboxed"), s
assert isinstance(s["data"]["execution_mode_degraded"], bool), s

i = call("system", "yai.workspace.inspect")
assert i["status"] == "ok", i
env = i["data"]["security"]
assert env["level_declared"] in ("scoped", "logical", "isolated", "sandboxed"), i
assert env["level_effective"] in ("scoped", "logical", "isolated", "sandboxed"), i
assert env["capabilities"]["sandbox_ready"] is True, i
assert env["scopes"]["filesystem"] is True, i
assert env["scopes"]["runtime_route"] is True, i
exe = i["data"]["execution"]
assert exe["mode_requested"] in ("scoped", "logical", "isolated", "sandboxed"), i
assert exe["mode_effective"] in ("scoped", "logical", "isolated", "sandboxed"), i
assert isinstance(exe["degraded"], bool), i

p = call("system", "yai.workspace.policy_effective")
assert p["status"] == "ok", p
assert p["data"]["security_level_effective"] in ("scoped", "logical", "isolated", "sandboxed"), p
assert p["data"]["execution_mode_effective"] in ("scoped", "logical", "isolated", "sandboxed"), p

d = call("system", "yai.workspace.debug_resolution")
assert d["status"] == "ok", d
assert d["data"]["security_level_effective"] in ("scoped", "logical", "isolated", "sandboxed"), d
assert d["data"]["execution_mode_effective"] in ("scoped", "logical", "isolated", "sandboxed"), d
PY

"$REPO"/tools/dev/validate_workspace_structure.py "$WS"

echo "workspace_security_envelope_v1: ok"
