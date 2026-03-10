#!/usr/bin/env bash
set -euo pipefail

REPO="$(cd "$(dirname "${BASH_SOURCE[0]}")/../../.." && pwd)"
YAI="$REPO/build/bin/yai"
SOCK="${YAI_RUNTIME_INGRESS:-$HOME/.yai/run/control.sock}"
WS="ws_digital_outbound_stage"
OBJ="enterprise.ecohmedia.digital-outbound.review-gate"
BIND_FILE="$HOME/.yai/session/active_workspace.json"

make -C "$REPO" yai >/dev/null
make -C "$REPO" law-embed-sync >/dev/null

"$YAI" down >/dev/null 2>&1 || true
rm -f "$SOCK" >/dev/null 2>&1 || true
rm -f "$BIND_FILE" >/dev/null 2>&1 || true
rm -rf "$HOME/.yai/run/$WS" >/dev/null 2>&1 || true

cleanup() {
  if [[ -n "${RUNTIME_PID:-}" ]] && kill -0 "$RUNTIME_PID" 2>/dev/null; then
    kill "$RUNTIME_PID" >/dev/null 2>&1 || true
    wait "$RUNTIME_PID" >/dev/null 2>&1 || true
  fi
  "$YAI" down --force >/dev/null 2>&1 || true
}
trap cleanup EXIT

(cd "$REPO" && "$YAI" >/tmp/yai_workspace_governance_persistence_dp5.log 2>&1) &
RUNTIME_PID=$!

for _ in $(seq 1 120); do
  [[ -S "$SOCK" ]] && break
  sleep 0.1
done
[[ -S "$SOCK" ]] || { echo "workspace_governance_persistence_dp5_v1: FAIL (missing ingress socket)"; exit 1; }

python3 - "$SOCK" "$WS" "$OBJ" "$HOME" <<'PY'
import json
import os
import socket
import struct
import sys

SOCK = sys.argv[1]
WS = sys.argv[2]
OBJ = sys.argv[3]
HOME = sys.argv[4]

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


def last_ndjson(path):
    with open(path, "r", encoding="utf-8") as f:
        lines = [ln.strip() for ln in f.readlines() if ln.strip()]
    assert lines, path
    return json.loads(lines[-1])


r = call(WS, "yai.workspace.create", [WS]); assert r["status"] == "ok", r
r = call("system", "yai.workspace.set", [WS]); assert r["status"] == "ok", r
r = call("system", "yai.workspace.domain_set", ["--family", "digital", "--specialization", "remote-publication"])
if not (r["status"] == "ok" or (r["status"] == "error" and r.get("reason") == "family_not_found")):
    raise AssertionError(r)

attached_obj = OBJ
r = call("system", "yai.workspace.policy_attach", [attached_obj])
if r["status"] == "error" and r.get("reason") == "governable_object_not_found":
    attached_obj = "customer.default.org-workspace-contextual-review"
    r = call("system", "yai.workspace.policy_attach", [attached_obj])
assert r["status"] == "ok", r
r = call("system", "yai.workspace.policy_activate", [attached_obj]); assert r["status"] == "ok", r

r = call(WS, "yai.workspace.run", ["digital.publish", "sink=external_untrusted", "contract=missing", "artifact=bundle-v1"])
assert r["status"] in ("ok", "error"), r

i = call("system", "yai.workspace.inspect")
assert i["status"] == "ok", i
gp = i["data"].get("governance_persistence")
assert isinstance(gp, dict), i
assert gp.get("last_attachment_ref", "").startswith("gatt-"), gp

p = call("system", "yai.workspace.policy_effective")
assert p["status"] == "ok", p
gp2 = p["data"].get("governance_persistence")
assert isinstance(gp2, dict), p

base = os.path.join(HOME, ".yai", "run", WS, "governance")
objects_log = os.path.join(base, "object-state.v1.ndjson")
lifecycle_log = os.path.join(base, "lifecycle-state.v1.ndjson")
attachments_log = os.path.join(base, "attachment-state.v1.ndjson")
index_path = os.path.join(base, "index.v1.json")

for path in (objects_log, lifecycle_log, attachments_log, index_path):
    assert os.path.exists(path), path

with open(index_path, "r", encoding="utf-8") as f:
    idx = json.load(f)

assert idx["type"] == "yai.governance.persistence.index.v1", idx
assert idx["workspace_id"] == WS, idx
assert idx["last_governance_object_ref"].startswith("gobj-"), idx
assert idx["last_lifecycle_ref"].startswith("glc-"), idx
assert idx["last_attachment_ref"].startswith("gatt-"), idx
assert idx["last_decision_ref"].startswith("dec-"), idx
assert idx["last_evidence_ref"].startswith("evd-"), idx

obj = last_ndjson(objects_log)
lc = last_ndjson(lifecycle_log)
att = last_ndjson(attachments_log)

assert obj["type"] == "yai.governance_object_state.v1", obj
assert obj["governance_object_id"] == attached_obj, obj
assert lc["type"] == "yai.governance_lifecycle_state.v1", lc
assert lc["governance_object_id"] == attached_obj, lc
assert att["type"] == "yai.governance_attachment_state.v1", att
assert att["governance_object_id"] == attached_obj, att
assert att["workspace_id"] == WS, att
assert att["decision_ref"].startswith("dec-"), att
assert att["evidence_ref"].startswith("evd-"), att
assert att["event_ref"].startswith("evt-"), att

r = call("system", "yai.workspace.unset")
assert r["status"] == "ok", r
PY

echo "workspace_governance_persistence_dp5_v1: ok"
