#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/../../../.." && pwd)"
source "$ROOT/tests/integration/qualification/lib/qualification_common.sh"
source "$ROOT/tests/integration/qualification/lib/qualification_asserts.sh"

yai_qual_require_bins

FIX_ROOT="$ROOT/tests/integration/qualification/fixtures/bologna-mini"
yai_qual_assert_dir "$FIX_ROOT/peer-a-performance" "ql_lan_distinct_assets_v1: missing fixture peer-a"
yai_qual_assert_dir "$FIX_ROOT/peer-b-programmazione" "ql_lan_distinct_assets_v1: missing fixture peer-b"

TMP_HOME="$(yai_qual_new_home)"
SOCK="$TMP_HOME/.yai/run/control.sock"
LOG_FILE="$TMP_HOME/owner-runtime.log"
WS="qw1_lan_distinct"
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
[[ -S "$SOCK" ]] || yai_qual_fail "ql_lan_distinct_assets_v1: owner socket not ready"

HOME="$TMP_HOME" YAI_RUNTIME_INGRESS="$SOCK" python3 - "$FIX_ROOT" <<'PY'
import json
import os
import socket
import struct
import pathlib
import hashlib
import sys

FIX_ROOT = pathlib.Path(sys.argv[1])
SOCK = os.environ["YAI_RUNTIME_INGRESS"]
WS = "qw1_lan_distinct"
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


def digest(path: pathlib.Path) -> str:
    return hashlib.sha256(path.read_bytes()).hexdigest()[:24]

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


def enroll_attach(label, coverage):
    enr = call(WS, {
      "type":"yai.control.call.v1",
      "command_id":"yai.source.enroll",
      "target_plane":"runtime",
      "workspace_id":WS,
      "source_label":label,
      "owner_ref":"unix://lan-owner"
    }, f"enroll-{label}")
    expect_ok(enr, f"enroll-{label}")
    node = enr["data"]["source_node_id"]
    tok = enr["data"]["owner_trust_artifact_token"]
    aid = enr["data"]["owner_trust_artifact_id"]
    att = call(WS, {
      "type":"yai.control.call.v1",
      "command_id":"yai.source.attach",
      "target_plane":"runtime",
      "workspace_id":WS,
      "source_node_id":node,
      "owner_trust_artifact_id":aid,
      "owner_trust_artifact_token":tok,
      "coverage_ref":coverage,
      "overlap_state":"none",
      "binding_scope":"workspace"
    }, f"attach-{label}")
    expect_ok(att, f"attach-{label}")
    return node, aid, tok, att["data"]["source_binding_id"]

node_a, aid_a, tok_a, binding_a = enroll_attach("p1", "coverage://lan/p1")
node_b, aid_b, tok_b, binding_b = enroll_attach("p2", "coverage://lan/p2")

asset_a = FIX_ROOT / "peer-a-performance" / "kpi_2025.csv"
asset_b = FIX_ROOT / "peer-b-programmazione" / "bilancio_previsionale.csv"

for idx, (node, aid, tok, binding, asset, idem) in enumerate([
    (node_a, aid_a, tok_a, binding_a, asset_a, "distinct-a"),
    (node_b, aid_b, tok_b, binding_b, asset_b, "distinct-b"),
], start=1):
    fp = digest(asset)
    sid = f"sa-distinct-{idx}"
    eid = f"se-distinct-{idx}"
    emit = call(WS, {
      "type":"yai.control.call.v1",
      "command_id":"yai.source.emit",
      "target_plane":"runtime",
      "workspace_id":WS,
      "source_node_id":node,
      "source_binding_id":binding,
      "owner_trust_artifact_id":aid,
      "owner_trust_artifact_token":tok,
      "idempotency_key": idem,
      "source_assets":[{
        "type":"yai.source_asset.v1",
        "source_asset_id": sid,
        "source_binding_id": binding,
        "locator": f"file://{asset}",
        "asset_type": "file",
        "provenance_fingerprint": f"sha256:{fp}",
        "observation_state": "observed"
      }],
      "source_acquisition_events":[{
        "type":"yai.source_acquisition_event.v1",
        "source_acquisition_event_id": eid,
        "source_node_id": node,
        "source_binding_id": binding,
        "source_asset_id": sid,
        "event_type": "discovered",
        "observed_at_epoch": 1773300000 + idx,
        "idempotency_key": idem,
        "delivery_status": "received"
      }]
    }, f"emit-{idx}")
    expect_ok(emit, f"emit-{idx}")

cov = call(WS, {
  "type":"yai.control.call.v1",
  "command_id":"yai.workspace.query",
  "target_plane":"runtime",
  "argv":["source.coverage"]
}, "q-coverage")
expect_ok(cov, "workspace.query source.coverage")
coverage = cov.get("data", {}).get("coverage", {})
if coverage.get("coverage_scope_count", 0) < 2:
    raise RuntimeError(f"expected coverage_scope_count>=2 got {cov}")
if coverage.get("overlap_count", -1) != 0:
    raise RuntimeError(f"expected overlap_count=0 for distinct assets got {cov}")

peer = call(WS, {
  "type":"yai.control.call.v1",
  "command_id":"yai.workspace.query",
  "target_plane":"runtime",
  "argv":["source.peer"]
}, "q-peer")
expect_ok(peer, "workspace.query source.peer")
rows = peer.get("data", {}).get("rows", [])
if len(rows) < 2:
    raise RuntimeError(f"expected >=2 peer rows got {peer}")
PY

echo "ql_lan_distinct_assets_v1: ok"
