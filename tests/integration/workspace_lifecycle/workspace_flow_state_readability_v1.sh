#!/usr/bin/env bash
set -euo pipefail

REPO="$(cd "$(dirname "${BASH_SOURCE[0]}")/../../.." && pwd)"
YAI="$REPO/build/bin/yai"
SOCK="${YAI_RUNTIME_INGRESS:-$HOME/.yai/run/control.sock}"
WS="ws_flow_state_v1"
BIND_FILE="$HOME/.yai/session/active_workspace.json"

if [[ ! -x "$YAI" ]]; then
  make -C "$REPO" yai >/dev/null
fi

"$YAI" down >/dev/null 2>&1 || true
rm -f "$SOCK" >/dev/null 2>&1 || true
rm -f "$BIND_FILE" >/dev/null 2>&1 || true

RUNTIME_PID=""
cleanup() {
  if [[ -n "$RUNTIME_PID" ]] && kill -0 "$RUNTIME_PID" 2>/dev/null; then
    kill "$RUNTIME_PID" >/dev/null 2>&1 || true
    wait "$RUNTIME_PID" >/dev/null 2>&1 || true
  fi
}
trap cleanup EXIT

(cd "$REPO" && "$YAI" >/tmp/yai_workspace_flow_state_runtime.log 2>&1) &
RUNTIME_PID=$!

for _ in $(seq 1 80); do
  [[ -S "$SOCK" ]] && break
  sleep 0.1
done
[[ -S "$SOCK" ]] || { echo "workspace_flow_state_readability_v1: FAIL (missing ingress socket)"; exit 1; }

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

    payload_obj = {
        "type": "yai.control.call.v1",
        "command_id": command_id,
        "target_plane": "runtime",
        "argv": argv,
    }
    payload = json.dumps(payload_obj).encode("utf-8")
    s.sendall(build(YAI_CMD_CONTROL_CALL, ws_id, payload, "call"))

    env = recv_exact(s, 96)
    _, _, _, _, cmd, _, _, _, plen, _ = struct.unpack(ENV_FMT, env)
    if cmd != YAI_CMD_CONTROL_CALL:
        raise RuntimeError("bad control response")
    body = recv_exact(s, plen).decode("utf-8")
    s.close()
    return json.loads(body)


def assert_op_state(obj):
    assert "operational_state" in obj, obj
    op = obj["operational_state"]
    for k in [
        "binding_state",
        "attached_governance_objects",
        "active_effective_stack",
        "last_event_ref",
        "last_flow_stage",
        "last_business_specialization",
        "last_enforcement_specialization",
        "last_effect",
        "last_authority",
        "last_evidence",
        "last_trace_ref",
        "review_state",
        "operational_summary",
    ]:
        assert k in op, (k, op)
    assert op["binding_state"] in ("active", "no_active", "stale", "invalid"), op


r = call(WS, "yai.workspace.create", [WS])
assert r["status"] == "ok", r
r = call("system", "yai.workspace.set", [WS])
assert r["status"] == "ok", r

r = call("system", "yai.workspace.policy_attach", ["customer.default.org-workspace-contextual-review"])
assert r["status"] == "ok", r

r = call("system", "yai.workspace.domain_set", ["--family", "digital", "--specialization", "remote-publication"])
assert r["status"] == "ok", r

r = call(WS, "yai.workspace.run", [
    "digital.publish",
    "sink=external_untrusted",
    "artifact=bundle-v1"
])
assert r["status"] in ("ok", "error"), r
assert_op_state(r["data"])

i = call("system", "yai.workspace.inspect")
assert i["status"] == "ok", i
assert_op_state(i["data"])
assert "customer.default.org-workspace-contextual-review" in i["data"]["operational_state"]["attached_governance_objects"], i

p = call("system", "yai.workspace.policy_effective")
assert p["status"] == "ok", p
assert_op_state(p["data"])
assert p["data"]["operational_state"]["active_effective_stack"], p

d = call("system", "yai.workspace.debug_resolution")
assert d["status"] == "ok", d
assert_op_state(d["data"])
assert d["data"]["operational_state"]["last_trace_ref"], d

r = call("system", "yai.workspace.unset")
assert r["status"] == "ok", r
PY

echo "workspace_flow_state_readability_v1: ok"
