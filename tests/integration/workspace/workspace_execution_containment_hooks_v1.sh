#!/usr/bin/env bash
set -euo pipefail

REPO="$(cd "$(dirname "${BASH_SOURCE[0]}")/../../.." && pwd)"
YAI="$REPO/build/bin/yai"
SOCK="${YAI_RUNTIME_INGRESS:-$HOME/.yai/run/control.sock}"
WS_SCOPED="ws_exec_scoped"
WS_ISO="ws_exec_iso"
WS_SBX="ws_exec_sbx"

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

(cd "$REPO" && "$YAI" >/tmp/yai_workspace_execution_hooks_runtime.log 2>&1) &
RUNTIME_PID=$!

for _ in $(seq 1 50); do
  [[ -S "$SOCK" ]] && break
  sleep 0.1
done
[[ -S "$SOCK" ]] || { echo "workspace_execution_containment_hooks_v1: FAIL (missing ingress socket)"; exit 1; }

python3 - "$SOCK" "$WS_SCOPED" "$WS_ISO" "$WS_SBX" <<'PY'
import json
import socket
import struct
import sys

SOCK = sys.argv[1]
WS_SCOPED = sys.argv[2]
WS_ISO = sys.argv[3]
WS_SBX = sys.argv[4]

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

# scoped (supported)
r = call(WS_SCOPED, "yai.workspace.create", [WS_SCOPED, "--containment-level", "scoped"])
assert r["status"] == "ok", r
assert r["data"]["execution_mode_requested"] == "scoped", r
assert r["data"]["execution_mode_effective"] == "scoped", r
assert r["data"]["execution_mode_degraded"] is False, r
r = call("system", "yai.workspace.set", [WS_SCOPED])
assert r["status"] == "ok", r
s = call("system", "yai.workspace.status")
assert s["status"] == "ok", s
assert s["data"]["execution_mode_requested"] == "scoped", s
assert s["data"]["execution_mode_effective"] == "scoped", s
assert s["data"]["execution_mode_degraded"] is False, s

# isolated (currently degraded to scoped)
r = call(WS_ISO, "yai.workspace.create", [WS_ISO, "--containment-level", "isolated"])
assert r["status"] == "ok", r
r = call("system", "yai.workspace.switch", [WS_ISO])
assert r["status"] == "ok", r
s = call("system", "yai.workspace.status")
assert s["status"] == "ok", s
assert s["data"]["execution_mode_requested"] == "isolated", s
assert s["data"]["execution_mode_effective"] == "scoped", s
assert s["data"]["execution_mode_degraded"] is True, s
assert s["data"]["execution_degraded_reason"] == "isolated_scopes_not_enforced", s

# sandboxed (currently degraded to scoped)
r = call(WS_SBX, "yai.workspace.create", [WS_SBX, "--containment-level", "sandboxed"])
assert r["status"] == "ok", r
r = call("system", "yai.workspace.switch", [WS_SBX])
assert r["status"] == "ok", r
s = call("system", "yai.workspace.status")
assert s["status"] == "ok", s
assert s["data"]["execution_mode_requested"] == "sandboxed", s
assert s["data"]["execution_mode_effective"] == "scoped", s
assert s["data"]["execution_mode_degraded"] is True, s
assert s["data"]["execution_degraded_reason"] == "sandbox_backend_unavailable", s

# run path must expose execution descriptor fields
r = call("system", "yai.workspace.switch", [WS_SCOPED])
assert r["status"] == "ok", r
r = call("system", "yai.workspace.domain_set", ["--family", "economic", "--specialization", "payments"])
assert r["status"] == "ok", r
r = call(WS_SCOPED, "yai.workspace.run", [
    "payment.authorize",
    "provider=bank",
    "resource=money-transfer",
    "amount=1250",
    "authority=supervisor"
])
assert r["status"] in ("ok", "error"), r
assert "execution" in r["data"], r
assert r["data"]["execution"]["mode_requested"] == "scoped", r
assert r["data"]["execution"]["mode_effective"] == "scoped", r
assert r["data"]["execution"]["attach_descriptor_ref"] != "", r
assert r["data"]["execution"]["execution_profile_ref"] != "", r

i = call("system", "yai.workspace.inspect")
assert i["status"] == "ok", i
assert i["data"]["execution"]["mode_requested"] == "scoped", i
assert i["data"]["execution"]["mode_effective"] == "scoped", i
assert i["data"]["execution"]["attach_descriptor_ref"] != "", i
assert i["data"]["execution"]["execution_profile_ref"] != "", i
PY

"$REPO"/tools/dev/validate_workspace_structure.py "$WS_SCOPED" "$WS_ISO" "$WS_SBX"

echo "workspace_execution_containment_hooks_v1: ok"
