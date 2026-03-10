#!/usr/bin/env bash
set -euo pipefail

REPO="$(cd "$(dirname "${BASH_SOURCE[0]}")/../../.." && pwd)"
YAI="$REPO/build/bin/yai"
SOCK="${YAI_RUNTIME_INGRESS:-$HOME/.yai/run/control.sock}"
# Must match the review-gate workspace targets declared in law examples.
WS="ws_digital_outbound_stage"
BIND_FILE="$HOME/.yai/session/active_workspace.json"
ATTACH_OBJ="enterprise.ecohmedia.digital-outbound.review-gate"

make -C "$REPO" yai >/dev/null
make -C "$REPO" law-embed-sync >/dev/null

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

(cd "$REPO" && "$YAI" >/tmp/yai_workspace_governed_vertical_slice_runtime.log 2>&1) &
RUNTIME_PID=$!

for _ in $(seq 1 100); do
  [[ -S "$SOCK" ]] && break
  sleep 0.1
done
[[ -S "$SOCK" ]] || { echo "workspace_governed_vertical_slice_v1: FAIL (missing ingress socket)"; exit 1; }

python3 - "$SOCK" "$WS" "$ATTACH_OBJ" <<'PY'
import json
import socket
import struct
import sys

SOCK = sys.argv[1]
WS = sys.argv[2]
ATTACH_OBJ = sys.argv[3]

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


def assert_event_surface(data):
    assert "event_surface" in data, data
    ev = data["event_surface"]
    for key in [
        "declared_scenario_specialization",
        "business_specialization",
        "enforcement_specialization",
        "flow_stage",
        "external_effect_boundary",
    ]:
        assert key in ev, (key, ev)


def assert_operational_state(data):
    assert "operational_state" in data, data
    op = data["operational_state"]
    assert op.get("binding_state") == "active", op
    assert "operational_summary" in op and op["operational_summary"], op
    assert "review_state" in op and op["review_state"], op


# 1) workspace bootstrap
r = call(WS, "yai.workspace.create", [WS])
assert r["status"] == "ok", r
r = call("system", "yai.workspace.set", [WS])
assert r["status"] == "ok", r

# 2) explicit governance attachment
r = call("system", "yai.workspace.policy_attach", [ATTACH_OBJ])
assert r["status"] == "ok", r
assert ATTACH_OBJ in r["data"]["policy_attachments"], r

# 3) declared business context
r = call("system", "yai.workspace.domain_set", ["--family", "digital", "--specialization", "remote-publication"])
assert r["status"] == "ok", r

# 4) event A: deny/quarantine expected
r = call(WS, "yai.workspace.run", [
    "digital.publish",
    "sink=external_untrusted",
    "artifact=bundle-v1"
])
assert r["status"] in ("ok", "error"), r
assert r["data"]["decision"]["family_id"] == "digital", r
assert_event_surface(r["data"])
assert_operational_state(r["data"])
assert r["data"]["event_surface"]["declared_scenario_specialization"] == "remote-publication", r

# 5) event B: stronger context (review/allow expected)
r = call(WS, "yai.workspace.run", [
    "digital.publish",
    "sink=internal_trusted",
    "contract=approved",
    "destination=ops_portal",
    "artifact=bundle-v1"
])
assert r["status"] in ("ok", "error"), r
assert_event_surface(r["data"])
assert_operational_state(r["data"])
if r["data"]["event_surface"]["enforcement_specialization"] == "network-egress":
    assert r["data"]["event_surface"]["business_specialization"] == "remote-publication", r

# 6) inspect/policy/debug expose same governed state
i = call("system", "yai.workspace.inspect")
assert i["status"] == "ok", i
assert_event_surface(i["data"])
assert_operational_state(i["data"])
assert ATTACH_OBJ in i["data"]["operational_state"]["attached_governance_objects"], i

p = call("system", "yai.workspace.policy_effective")
assert p["status"] == "ok", p
assert_event_surface(p["data"])
assert_operational_state(p["data"])
assert ATTACH_OBJ in p["data"]["operational_state"]["attached_governance_objects"], p
assert p["data"]["operational_state"]["active_effective_stack"], p

d = call("system", "yai.workspace.debug_resolution")
assert d["status"] == "ok", d
assert_event_surface(d["data"])
assert_operational_state(d["data"])
assert d["data"]["operational_state"]["last_trace_ref"], d

# cleanup
r = call("system", "yai.workspace.unset")
assert r["status"] == "ok", r
PY

echo "workspace_governed_vertical_slice_v1: ok"
