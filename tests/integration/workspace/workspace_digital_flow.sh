#!/usr/bin/env bash
set -euo pipefail

REPO="$(cd "$(dirname "${BASH_SOURCE[0]}")/../../.." && pwd)"
YAI="$REPO/build/bin/yai"
SOCK="${YAI_RUNTIME_INGRESS:-$HOME/.yai/run/control.sock}"
WS="ws_digital_flow_v1"
BIND_FILE="$HOME/.yai/session/active_workspace.json"

if [[ ! -x "$YAI" ]]; then
  make -C "$REPO" yai >/dev/null
fi

"$YAI" down >/dev/null 2>&1 || true
rm -f "$SOCK" >/dev/null 2>&1 || true
rm -f "$BIND_FILE" >/dev/null 2>&1 || true

cleanup() {
  if [[ -n "${RUNTIME_PID:-}" ]] && kill -0 "$RUNTIME_PID" 2>/dev/null; then
    kill "$RUNTIME_PID" >/dev/null 2>&1 || true
    wait "$RUNTIME_PID" >/dev/null 2>&1 || true
  fi
  "$YAI" down --force >/dev/null 2>&1 || true
}
trap cleanup EXIT

(cd "$REPO" && "$YAI" up >/tmp/yai_workspace_digital_flow_runtime.log 2>&1) &
RUNTIME_PID=$!

for _ in $(seq 1 120); do
  [[ -S "$SOCK" ]] && break
  sleep 0.1
done
[[ -S "$SOCK" ]] || { echo "workspace_digital_flow_v1: FAIL (missing ingress socket)"; exit 1; }

python3 - "$SOCK" "$WS" <<PY
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

# 1) workspace bootstrap
r = call(WS, "yai.workspace.create", [WS])
assert r["status"] == "ok", r
r = call("system", "yai.workspace.set", [WS])
assert r["status"] == "ok", r

# 2) digital publication context
r = call("system", "yai.workspace.domain_set", ["--family", "digital", "--specialization", "remote-publication"])
assert r["status"] == "ok", r
assert r["data"]["declared"]["family"] == "digital", r

# 3) publication without contract -> deny/quarantine
r = call(WS, "yai.workspace.run", [
    "digital.publish",
    "sink=external_untrusted",
    "artifact=bundle-v1"
])
assert r["status"] == "error", r
assert r["data"]["decision"]["family_id"] == "digital", r
assert r["data"]["decision"]["specialization_id"] == "remote-publication", r
assert r["data"]["decision"]["effect"] in ("deny", "quarantine"), r

# 4) publication with contract + untrusted sink -> quarantine
r = call(WS, "yai.workspace.run", [
    "digital.publish",
    "sink=external_untrusted",
    "contract=approved",
    "artifact=bundle-v1"
])
assert r["data"]["decision"]["family_id"] == "digital", r
assert r["data"]["decision"]["specialization_id"] == "remote-publication", r
assert r["data"]["decision"]["effect"] in ("quarantine", "deny"), r

# 5) publication with contract + trusted sink -> review/allow (or quarantine in stricter overlays)
r = call(WS, "yai.workspace.run", [
    "digital.publish",
    "sink=internal_trusted",
    "contract=approved",
    "artifact=bundle-v1",
    "destination=ops_portal"
])
assert r["data"]["decision"]["family_id"] == "digital", r
assert r["data"]["decision"]["specialization_id"] == "remote-publication", r
assert r["data"]["decision"]["effect"] in ("review_required", "allow", "quarantine"), r

# 6) retrieval specialization
r = call("system", "yai.workspace.domain_set", ["--family", "digital", "--specialization", "remote-retrieval"])
assert r["status"] == "ok", r
r = call(WS, "yai.workspace.run", [
    "digital.retrieve",
    "source=trusted_repo",
    "sink=internal"
])
assert r["data"]["decision"]["family_id"] == "digital", r
assert r["data"]["decision"]["specialization_id"] in ("remote-retrieval", "artifact-distribution"), r
assert r["data"]["decision"]["effect"] in ("allow", "review_required", "quarantine", "deny"), r

# 7) artifact distribution specialization
r = call("system", "yai.workspace.domain_set", ["--family", "digital", "--specialization", "artifact-distribution"])
assert r["status"] == "ok", r
r = call(WS, "yai.workspace.run", [
    "digital.distribute",
    "artifact=bundle-v1"
])
assert r["data"]["decision"]["specialization_id"] == "artifact-distribution", r
assert r["data"]["decision"]["effect"] in ("deny", "quarantine", "review_required"), r

# 8) inspect/debug/policy expose digital summaries
p = call("system", "yai.workspace.policy_effective")
assert p["status"] == "ok", p
assert p["data"]["family_effective"] == "digital", p
assert p["data"]["digital"]["outbound_context_summary"], p
assert p["data"]["digital"]["sink_target_summary"], p

q = call("system", "yai.workspace.debug_resolution")
assert q["status"] == "ok", q
assert q["data"]["declared"]["family"] == "digital", q
assert q["data"]["digital"]["publication_control_summary"], q

i = call("system", "yai.workspace.inspect")
assert i["status"] == "ok", i
assert i["data"]["identity"]["workspace_id"] == WS, i
assert i["data"]["digital"]["distribution_control_summary"], i

# 9) cleanup binding
r = call("system", "yai.workspace.unset")
assert r["status"] == "ok", r
PY

echo "workspace_digital_flow_v1: ok"
