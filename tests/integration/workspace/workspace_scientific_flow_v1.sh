#!/usr/bin/env bash
set -euo pipefail

REPO="$(cd "$(dirname "${BASH_SOURCE[0]}")/../../.." && pwd)"
YAI="$REPO/build/bin/yai"
SOCK="${YAI_RUNTIME_INGRESS:-$HOME/.yai/run/control.sock}"
WS="ws_scientific_flow_v1"
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

(cd "$REPO" && "$YAI" up >/tmp/yai_workspace_scientific_flow_runtime.log 2>&1) &
RUNTIME_PID=$!

for _ in $(seq 1 120); do
  [[ -S "$SOCK" ]] && break
  sleep 0.1
done
[[ -S "$SOCK" ]] || { echo "workspace_scientific_flow_v1: FAIL (missing ingress socket)"; exit 1; }

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

# 1) workspace bootstrap
r = call(WS, "yai.workspace.create", [WS])
assert r["status"] == "ok", r
r = call("system", "yai.workspace.set", [WS])
assert r["status"] == "ok", r

# 2) scientific domain context
r = call("system", "yai.workspace.domain_set", ["--family", "scientific", "--specialization", "parameter-governance"])
assert r["status"] == "ok", r
assert r["data"]["declared"]["family"] == "scientific", r
assert r["data"]["declared"]["specialization"] == "parameter-governance", r

# 3) parameter mutation without lock -> deny
r = call(WS, "yai.workspace.run", [
    "experiment.parameters.update",
    "dataset=chem-v3",
    "result=run-17"
])
assert r["status"] == "error", r
assert r["data"]["decision"]["family_id"] == "scientific", r
assert r["data"]["decision"]["specialization_id"] == "parameter-governance", r
assert r["data"]["decision"]["effect"] == "deny", r

# 4) publication control with contract but incomplete reproducibility -> quarantine/error
r = call(WS, "yai.workspace.run", [
    "experiment.result.publish",
    "dataset=chem-v3",
    "result=run-17",
    "contract=approved"
])
assert r["data"]["decision"]["family_id"] == "scientific", r
assert r["data"]["decision"]["specialization_id"] in ("result-publication-control", "parameter-governance"), r
assert r["data"]["decision"]["effect"] in ("quarantine", "deny", "review_required", "allow"), r

# 5) publication with reproducibility proofpack + contract -> review_required
r = call(WS, "yai.workspace.run", [
    "experiment.result.publish",
    "dataset=chem-v3",
    "result=run-17",
    "contract=approved",
    "params_hash=cfg-22",
    "proofpack=sha256:abc",
    "lineage=dataset:chem-v3/run:17"
])
assert r["data"]["decision"]["family_id"] == "scientific", r
assert r["data"]["decision"]["specialization_id"] in ("result-publication-control", "parameter-governance"), r
assert r["data"]["decision"]["effect"] in ("review_required", "allow", "quarantine", "deny"), r

# 6) inspect/debug/policy must expose scientific summaries
p = call("system", "yai.workspace.policy_effective")
assert p["status"] == "ok", p
assert p["data"]["family_effective"] == "scientific", p
assert p["data"]["scientific"]["publication_control_summary"], p
assert p["data"]["scientific"]["reproducibility_summary"], p

q = call("system", "yai.workspace.debug_resolution")
assert q["status"] == "ok", q
assert q["data"]["declared"]["family"] == "scientific", q
assert q["data"]["scientific"]["parameter_governance_summary"], q

i = call("system", "yai.workspace.inspect")
assert i["status"] == "ok", i
assert i["data"]["identity"]["workspace_id"] == WS, i
assert i["data"]["scientific"]["experiment_context_summary"], i

# 7) cleanup binding
r = call("system", "yai.workspace.unset")
assert r["status"] == "ok", r
PY

echo "workspace_scientific_flow_v1: ok"
