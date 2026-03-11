#!/usr/bin/env bash
set -euo pipefail

REPO="$(cd "$(dirname "${BASH_SOURCE[0]}")/../../.." && pwd)"
YAI="$REPO/build/bin/yai"
SOCK="$HOME/.yai/run/control.sock"
WS="ws_authority_artifact_dp6_v1"
OBJ="enterprise.ecohmedia.digital-outbound.review-gate"
BIND_FILE="$HOME/.yai/session/active_workspace.json"

if [[ ! -x "$YAI" ]]; then
  make -C "$REPO" yai >/dev/null
fi
make -C "$REPO" law-embed-sync >/dev/null

env -u YAI_RUNTIME_INGRESS "$YAI" down >/dev/null 2>&1 || true
rm -f "$SOCK" >/dev/null 2>&1 || true
rm -f "$BIND_FILE" >/dev/null 2>&1 || true
rm -rf "$HOME/.yai/run/$WS" >/dev/null 2>&1 || true

cleanup() {
  if [[ -n "${RUNTIME_PID:-}" ]] && kill -0 "$RUNTIME_PID" 2>/dev/null; then
    kill "$RUNTIME_PID" >/dev/null 2>&1 || true
    wait "$RUNTIME_PID" >/dev/null 2>&1 || true
  fi
  env -u YAI_RUNTIME_INGRESS "$YAI" down --force >/dev/null 2>&1 || true
}
trap cleanup EXIT

(cd "$REPO" && env -u YAI_RUNTIME_INGRESS "$YAI" >/tmp/yai_workspace_authority_artifact_dp6.log 2>&1) &
RUNTIME_PID=$!

for _ in $(seq 1 120); do
  [[ -S "$SOCK" ]] && break
  sleep 0.1
done
[[ -S "$SOCK" ]] || { echo "workspace_authority_artifact_persistence_dp6_v1: FAIL (missing ingress socket)"; exit 1; }

python3 - "$SOCK" "$WS" "$OBJ" "$HOME" <<'PY'
import json
import os
import socket
import struct
import sys
import time

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
    if not lines:
        return None
    return json.loads(lines[-1])


r = call(WS, "yai.workspace.create", [WS]); assert r["status"] == "ok", r
r = call("system", "yai.workspace.set", [WS]); assert r["status"] == "ok", r

r = None
for _ in range(20):
    r = call("system", "yai.workspace.domain_set", ["--family", "digital", "--specialization", "remote-publication"])
    if r["status"] == "ok":
        break
    time.sleep(0.1)
assert r is not None and r["status"] == "ok", r

r = call(WS, "yai.workspace.run", ["digital.publish", "sink=external_untrusted", "contract=missing", "artifact=bundle-v1"])
assert r["status"] in ("ok", "error"), r

for command_id in ("yai.workspace.inspect", "yai.workspace.policy_effective", "yai.workspace.debug_resolution"):
    out = call("system", command_id)
    assert out["status"] == "ok", out
    ap = out["data"].get("authority_artifact_persistence")
    assert isinstance(ap, dict), (command_id, out)
    for key in (
        "last_authority_ref",
        "last_authority_resolution_ref",
        "last_artifact_ref",
        "last_artifact_linkage_ref",
        "authority_store_ref",
        "artifact_store_ref",
    ):
        assert key in ap, (command_id, ap)
    assert ap.get("authority_store_ref", ""), (command_id, ap)
    assert ap.get("artifact_store_ref", ""), (command_id, ap)

base = os.path.join(HOME, ".yai", "run", WS)
authority_log = os.path.join(base, "authority", "authority-state.v1.ndjson")
authority_resolution_log = os.path.join(base, "authority", "resolution-state.v1.ndjson")
authority_index = os.path.join(base, "authority", "index.v1.json")
artifact_metadata_log = os.path.join(base, "artifacts", "metadata.v1.ndjson")
artifact_linkage_log = os.path.join(base, "artifacts", "linkage.v1.ndjson")
artifact_index = os.path.join(base, "artifacts", "metadata.index.v1.json")

for path in (
    authority_log,
    authority_resolution_log,
    authority_index,
    artifact_metadata_log,
    artifact_linkage_log,
    artifact_index,
):
    assert os.path.exists(path), path

with open(authority_index, "r", encoding="utf-8") as f:
    aidx = json.load(f)
with open(artifact_index, "r", encoding="utf-8") as f:
    midx = json.load(f)

assert aidx["type"] == "yai.authority.index.v1", aidx
assert aidx["workspace_id"] == WS, aidx
for key in ("last_authority_ref", "last_resolution_ref", "last_decision_ref", "last_evidence_ref"):
    assert key in aidx, aidx

assert midx["type"] == "yai.artifact.metadata.index.v1", midx
assert midx["workspace_id"] == WS, midx
for key in ("last_artifact_ref", "last_linkage_ref", "last_decision_ref", "last_evidence_ref"):
    assert key in midx, midx

auth = last_ndjson(authority_log)
ares = last_ndjson(authority_resolution_log)
meta = last_ndjson(artifact_metadata_log)
link = last_ndjson(artifact_linkage_log)

if auth is not None:
    assert auth["type"] == "yai.authority_state.v1", auth
    assert auth["workspace_ref"] == WS, auth
if ares is not None:
    assert ares["type"] == "yai.authority_resolution_record.v1", ares
    assert ares["workspace_ref"] == WS, ares
if meta is not None:
    assert meta["type"] == "yai.artifact_metadata.v1", meta
    assert meta["workspace_ref"] == WS, meta
if link is not None:
    assert link["type"] == "yai.artifact_governance_linkage.v1", link
    assert link["workspace_ref"] == WS, link

r = call("system", "yai.workspace.unset")
assert r["status"] == "ok", r
PY

echo "workspace_authority_artifact_persistence_dp6_v1: ok"
