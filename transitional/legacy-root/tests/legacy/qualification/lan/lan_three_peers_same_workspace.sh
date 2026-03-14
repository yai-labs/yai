#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/../../../.." && pwd)"
source "$ROOT/tests/legacy/qualification/lib/qualification_common.sh"
source "$ROOT/tests/legacy/qualification/lib/qualification_asserts.sh"

yai_qual_require_bins

TMP_HOME="$(yai_qual_new_home)"
SOCK="$TMP_HOME/.yai/run/control.sock"
LOG_FILE="$TMP_HOME/owner-runtime.log"
WS="qw1_lan_3peers"
OWNER_PID=""

cleanup() {
  HOME="$TMP_HOME" YAI_RUNTIME_INGRESS="$SOCK" "$YAI_BIN" down >/dev/null 2>&1 || true
  if [[ -n "$OWNER_PID" ]] && kill -0 "$OWNER_PID" 2>/dev/null; then
    kill "$OWNER_PID" >/dev/null 2>&1 || true
    wait "$OWNER_PID" >/dev/null 2>&1 || true
  fi
  rm -rf "$TMP_HOME"
}
trap cleanup EXIT

mkdir -p "$TMP_HOME/.yai/run"
HOME="$TMP_HOME" YAI_RUNTIME_INGRESS="$SOCK" "$YAI_BIN" down >/dev/null 2>&1 || true
rm -f "$SOCK" >/dev/null 2>&1 || true
(cd "$ROOT" && HOME="$TMP_HOME" YAI_RUNTIME_INGRESS="$SOCK" "$YAI_BIN" >"$LOG_FILE" 2>&1) &
OWNER_PID=$!
for _ in $(seq 1 80); do
  [[ -S "$SOCK" ]] && break
  sleep 0.1
done
[[ -S "$SOCK" ]] || yai_qual_fail "lan_three_peers_same_workspace: owner socket not ready"

HOME="$TMP_HOME" YAI_RUNTIME_INGRESS="$SOCK" python3 - <<'PY'
import json
import os
import socket
import struct

SOCK = os.environ["YAI_RUNTIME_INGRESS"]
WS = "qw1_lan_3peers"
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


def call(ws_id, body, trace):
    s = socket.socket(socket.AF_UNIX, socket.SOCK_STREAM)
    s.connect(SOCK)
    hs_payload = struct.pack(REQ_FMT, YAI_PROTOCOL_IDS_VERSION, 0, b"yai-qw1")
    s.sendall(build(YAI_CMD_HANDSHAKE, ws_id, hs_payload, "hs-qw1"))
    env = recv_exact(s, 96)
    _, _, _, _, cmd, _, _, _, plen, _ = struct.unpack(ENV_FMT, env)
    if cmd != YAI_CMD_HANDSHAKE:
        raise RuntimeError("bad handshake")
    recv_exact(s, plen)
    payload = json.dumps(body).encode("utf-8")
    s.sendall(build(YAI_CMD_CONTROL_CALL, ws_id, payload, trace))
    env = recv_exact(s, 96)
    _, _, _, _, cmd, _, _, _, plen, _ = struct.unpack(ENV_FMT, env)
    if cmd != YAI_CMD_CONTROL_CALL:
        raise RuntimeError("bad control")
    out = recv_exact(s, plen).decode("utf-8")
    s.close()
    return json.loads(out)


def expect_ok(reply, why):
    if reply.get("status") != "ok":
        raise RuntimeError(f"{why}: expected ok got {reply}")


def enroll_attach(label, coverage, role):
    enr = call(WS, {
      "type":"yai.control.call.v1",
      "command_id":"yai.source.enroll",
      "target_plane":"runtime",
      "workspace_id":WS,
      "source_label":label,
      "owner_ref":"unix://lan-owner"
    }, f"enroll-{label}")
    expect_ok(enr, f"enroll-{label}")
    node = enr.get("data", {}).get("source_node_id")
    tok = enr.get("data", {}).get("owner_trust_artifact_token")
    aid = enr.get("data", {}).get("owner_trust_artifact_id")
    att = call(WS, {
      "type":"yai.control.call.v1",
      "command_id":"yai.source.attach",
      "target_plane":"runtime",
      "workspace_id":WS,
      "source_node_id":node,
      "owner_trust_artifact_id":aid,
      "owner_trust_artifact_token":tok,
      "peer_role":role,
      "peer_scope":coverage,
      "coverage_ref":coverage,
      "overlap_state":"none",
      "binding_scope":"workspace"
    }, f"attach-{label}")
    expect_ok(att, f"attach-{label}")

expect_ok(call("system", {
  "type":"yai.control.call.v1",
  "command_id":"yai.workspace.create",
  "target_plane":"runtime",
  "argv":[WS]
}, "create"), "workspace.create")
expect_ok(call("system", {
  "type":"yai.control.call.v1",
  "command_id":"yai.workspace.set",
  "target_plane":"runtime",
  "argv":[WS]
}, "set"), "workspace.set")

enroll_attach("p1", "coverage://lan/p1", "r1")
enroll_attach("p2", "coverage://lan/p2", "r2")
enroll_attach("p3", "coverage://lan/p3", "r3")

peer = call(WS, {
  "type":"yai.control.call.v1",
  "command_id":"yai.workspace.query",
  "target_plane":"runtime",
  "argv":["source.peer"]
}, "q-peer")
expect_ok(peer, "workspace.query source.peer")
rows = peer.get("data", {}).get("rows", [])
if len(rows) < 3:
    raise RuntimeError(f"expected >=3 peer rows got {peer}")

summary = call(WS, {
  "type":"yai.control.call.v1",
  "command_id":"yai.workspace.query",
  "target_plane":"runtime",
  "argv":["source"]
}, "q-source")
expect_ok(summary, "workspace.query source")
coord = summary.get("data", {}).get("coordination", {})
if coord.get("peer_count", 0) < 3:
    raise RuntimeError(f"expected peer_count>=3 got {summary}")
if summary.get("data", {}).get("summary", {}).get("workspace_peer_membership_count", 0) < 3:
    raise RuntimeError(f"expected workspace_peer_membership_count>=3 got {summary}")
if summary.get("data", {}).get("summary", {}).get("source_graph_node_count", 0) < 3:
    raise RuntimeError(f"expected source_graph_node_count>=3 got {summary}")
PY

echo "lan_three_peers_same_workspace: ok"
