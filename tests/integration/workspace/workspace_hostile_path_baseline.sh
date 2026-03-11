#!/usr/bin/env bash
set -euo pipefail

REPO="$(cd "$(dirname "${BASH_SOURCE[0]}")/../../.." && pwd)"
YAI="$REPO/build/bin/yai"
SOCK="${YAI_RUNTIME_INGRESS:-$HOME/.yai/run/control.sock}"
WS_A="ws_hostile_a"
WS_B="ws_hostile_b"
WS_DEG="ws_hostile_deg"

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

(cd "$REPO" && "$YAI" >/tmp/yai_workspace_hostile_baseline_runtime.log 2>&1) &
RUNTIME_PID=$!

for _ in $(seq 1 50); do
  [[ -S "$SOCK" ]] && break
  sleep 0.1
done
[[ -S "$SOCK" ]] || { echo "workspace_hostile_path_baseline_v1: FAIL (missing ingress socket)"; exit 1; }

python3 - "$SOCK" "$WS_A" "$WS_B" "$WS_DEG" <<'PY'
import json
import os
import socket
import struct
import sys

SOCK = sys.argv[1]
WS_A = sys.argv[2]
WS_B = sys.argv[3]
WS_DEG = sys.argv[4]

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

# 1) path escape baseline: reject traversal in root option
r = call("system", "yai.workspace.create", ["ws_escape_fail", "--root", "../tmp/escape"])
assert r["status"] == "error", r

# 2) setup two workspaces and bind A
r = call(WS_A, "yai.workspace.create", [WS_A]); assert r["status"] == "ok", r
r = call(WS_B, "yai.workspace.create", [WS_B]); assert r["status"] == "ok", r
r = call("system", "yai.workspace.set", [WS_A]); assert r["status"] == "ok", r
r = call("system", "yai.workspace.domain_set", ["--family", "economic", "--specialization", "payments"]); assert r["status"] == "ok", r
r = call(WS_A, "yai.workspace.run", ["payment.authorize", "provider=bank", "resource=money-transfer", "amount=125", "authority=reviewer"])
assert r["status"] in ("ok", "error"), r

# 3) cross-workspace contamination attempt must be denied
r = call(WS_B, "yai.runtime.ping", ["cross-scope"])
assert r["status"] == "error", r
assert r["reason"] == "cross_workspace_scope_denied", r

# 4) stale binding abuse: force invalid workspace id in binding file
bind_path = os.path.expanduser("~/.yai/session/active_workspace.json")
with open(bind_path, "w", encoding="utf-8") as f:
    json.dump({
        "type": "yai.workspace.binding.v1",
        "workspace_id": "bad/id",
        "workspace_alias": "bad",
        "bound_at": 0,
        "source": "forced_test"
    }, f)
r = call("system", "yai.workspace.current")
assert r["status"] == "ok", r
assert r["data"]["binding_status"] == "invalid", r
assert r["data"]["reason"] in ("invalid_workspace_id", "workspace_not_found", "binding_workspace_id_invalid"), r

# 5) degraded-mode abuse visibility: requested sandboxed must degrade clearly
r = call(WS_DEG, "yai.workspace.create", [WS_DEG, "--containment-level", "sandboxed"])
assert r["status"] == "ok", r
r = call("system", "yai.workspace.set", [WS_DEG]); assert r["status"] == "ok", r
s = call("system", "yai.workspace.status")
assert s["status"] == "ok", s
assert s["data"]["execution_mode_requested"] == "sandboxed", s
assert s["data"]["execution_mode_effective"] == "scoped", s
assert s["data"]["execution_mode_degraded"] is True, s
assert s["data"]["execution_degraded_reason"] == "sandbox_backend_unavailable", s

i = call("system", "yai.workspace.inspect")
assert i["status"] == "ok", i
assert i["data"]["execution"]["mode_requested"] == "sandboxed", i
assert i["data"]["execution"]["mode_effective"] == "scoped", i
assert i["data"]["execution"]["degraded"] is True, i
assert i["data"]["execution"]["degraded_reason"] == "sandbox_backend_unavailable", i
PY

"$REPO"/tools/dev/validate_workspace_structure.py "$WS_A" "$WS_B" "$WS_DEG"

echo "workspace_hostile_path_baseline_v1: ok"
