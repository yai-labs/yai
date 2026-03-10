#!/usr/bin/env bash
set -euo pipefail

REPO="$(cd "$(dirname "${BASH_SOURCE[0]}")/../../.." && pwd)"
YAI="$REPO/build/bin/yai"
SOCK="${YAI_RUNTIME_INGRESS:-$HOME/.yai/run/control.sock}"
WS_A="ws_iso_a"
WS_B="ws_iso_b"

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

(cd "$REPO" && "$YAI" >/tmp/yai_workspace_isolation_guards_runtime.log 2>&1) &
RUNTIME_PID=$!

for _ in $(seq 1 50); do
  [[ -S "$SOCK" ]] && break
  sleep 0.1
done
[[ -S "$SOCK" ]] || { echo "workspace_isolation_guards_v1: FAIL (missing ingress socket)"; exit 1; }

python3 - "$SOCK" "$WS_A" "$WS_B" <<'PY'
import json
import os
import socket
import struct
import sys

SOCK = sys.argv[1]
WS_A = sys.argv[2]
WS_B = sys.argv[3]

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

def call(ws_id, command_id, argv=None, extra=None):
    if argv is None:
        argv = []
    if extra is None:
        extra = {}

    s = socket.socket(socket.AF_UNIX, socket.SOCK_STREAM)
    s.connect(SOCK)

    hs_payload = struct.pack(REQ_FMT, YAI_PROTOCOL_IDS_VERSION, 0, b"yai-test")
    s.sendall(build(YAI_CMD_HANDSHAKE, ws_id, hs_payload, "hs"))
    env = recv_exact(s, 96)
    _, _, _, _, cmd, _, _, _, plen, _ = struct.unpack(ENV_FMT, env)
    if cmd != YAI_CMD_HANDSHAKE:
        raise RuntimeError("bad handshake")
    recv_exact(s, plen)

    payload_obj = {
        "type": "yai.control.call.v1",
        "command_id": command_id,
        "target_plane": "runtime",
        "argv": argv,
    }
    payload_obj.update(extra)

    payload = json.dumps(payload_obj).encode("utf-8")
    s.sendall(build(YAI_CMD_CONTROL_CALL, ws_id, payload, "call"))
    env = recv_exact(s, 96)
    _, _, _, _, cmd, _, _, _, plen, _ = struct.unpack(ENV_FMT, env)
    if cmd != YAI_CMD_CONTROL_CALL:
        raise RuntimeError("bad control response")
    body = recv_exact(s, plen).decode("utf-8")
    s.close()
    return json.loads(body)

# setup two workspaces
r = call(WS_A, "yai.workspace.create", [WS_A])
assert r["status"] == "ok", r
r = call(WS_B, "yai.workspace.create", [WS_B])
assert r["status"] == "ok", r

# bind to workspace A
r = call("system", "yai.workspace.set", [WS_A])
assert r["status"] == "ok", r

# run a workspace-scoped action on A
r = call("system", "yai.workspace.domain_set", ["--family", "economic", "--specialization", "payments"])
assert r["status"] == "ok", r
r = call(WS_A, "yai.workspace.run", ["payment.authorize", "provider=bank", "resource=money-transfer", "amount=50", "authority=reviewer"])
assert r["status"] in ("ok", "error"), r

# cross-workspace runtime control call should be denied while A is active
r = call(WS_B, "yai.runtime.ping", [])
assert r["status"] == "error", r
assert r["code"] == "BAD_ARGS", r
assert r["reason"] == "cross_workspace_scope_denied", r

# inspect remains scoped to active workspace A
r = call("system", "yai.workspace.inspect", [])
assert r["status"] == "ok", r
assert r["data"]["identity"]["workspace_id"] == WS_A, r
assert r["data"]["boundary"]["namespace_valid"] is True, r
assert r["data"]["boundary"]["state"] == "enforced", r

# tamper manifest namespace path => binding becomes invalid with explicit reason
manifest = os.path.expanduser(f"~/.yai/run/{WS_A}/manifest.json")
with open(manifest, "r", encoding="utf-8") as f:
    m = json.load(f)
m.setdefault("root_model", {})["runtime_state_root"] = f"/tmp/not-{WS_A}"
with open(manifest, "w", encoding="utf-8") as f:
    json.dump(m, f, indent=2)

r = call("system", "yai.workspace.current", [])
assert r["status"] == "ok", r
assert r["data"]["binding_status"] == "invalid", r
assert r["data"]["reason"] in ("runtime_state_root_mismatch", "metadata_root_mismatch", "workspace_namespace_invalid"), r
PY

echo "workspace_isolation_guards_v1: ok"
