#!/usr/bin/env bash
set -euo pipefail

REPO="$(cd "$(dirname "${BASH_SOURCE[0]}")/../../.." && pwd)"
YAI="$REPO/build/bin/yai"
TMP_HOME="$(mktemp -d "${TMPDIR:-/tmp}/yai_source_read_model.XXXXXX")"
SOCK="$TMP_HOME/.yai/run/control.sock"

if [[ ! -x "$YAI" ]]; then
  make -C "$REPO" yai >/dev/null
fi

mkdir -p "$TMP_HOME/.yai/run"
HOME="$TMP_HOME" YAI_RUNTIME_INGRESS="$SOCK" "$YAI" down >/dev/null 2>&1 || true
rm -f "$SOCK" >/dev/null 2>&1 || true

RUNTIME_PID=""
cleanup() {
  HOME="$TMP_HOME" YAI_RUNTIME_INGRESS="$SOCK" "$YAI" down >/dev/null 2>&1 || true
  if [[ -n "$RUNTIME_PID" ]] && kill -0 "$RUNTIME_PID" 2>/dev/null; then
    kill "$RUNTIME_PID" >/dev/null 2>&1 || true
    wait "$RUNTIME_PID" >/dev/null 2>&1 || true
  fi
  rm -rf "$TMP_HOME"
}
trap cleanup EXIT

(cd "$REPO" && HOME="$TMP_HOME" YAI_RUNTIME_INGRESS="$SOCK" "$YAI" >/tmp/yai_source_plane_read_model.log 2>&1) &
RUNTIME_PID=$!

for _ in $(seq 1 50); do
  [[ -S "$SOCK" ]] && break
  sleep 0.1
done
[[ -S "$SOCK" ]] || { echo "source_plane_read_model: FAIL (missing ingress socket)"; exit 1; }

HOME="$TMP_HOME" YAI_RUNTIME_INGRESS="$SOCK" python3 - <<'PY'
import json
import os
import socket
import struct

SOCK = os.environ.get("YAI_RUNTIME_INGRESS", os.path.expanduser("~/.yai/run/control.sock"))
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
    hs_payload = struct.pack(REQ_FMT, YAI_PROTOCOL_IDS_VERSION, 0, b"yai-yd6")
    s.sendall(build(YAI_CMD_HANDSHAKE, ws_id, hs_payload, "hs-yd6"))
    env = recv_exact(s, 96)
    _, _, _, _, cmd, _, _, _, plen, _ = struct.unpack(ENV_FMT, env)
    if cmd != YAI_CMD_HANDSHAKE:
        raise RuntimeError("bad handshake response")
    recv_exact(s, plen)

    payload = json.dumps(body).encode("utf-8")
    s.sendall(build(YAI_CMD_CONTROL_CALL, ws_id, payload, trace))
    env = recv_exact(s, 96)
    _, _, _, _, cmd, _, _, _, plen, _ = struct.unpack(ENV_FMT, env)
    if cmd != YAI_CMD_CONTROL_CALL:
        raise RuntimeError("bad control response")
    body = recv_exact(s, plen).decode("utf-8")
    s.close()
    try:
        return json.loads(body)
    except Exception as exc:
        raise RuntimeError(f"json decode failed trace={trace} plen={plen} body={body}") from exc

def expect_ok(reply, why):
    if reply.get("status") != "ok":
        raise RuntimeError(f"{why}: expected ok, got {reply}")

ws = "yd6_source_ws"
expect_ok(call("system", {
  "type":"yai.control.call.v1",
  "command_id":"yai.workspace.create",
  "target_plane":"runtime",
  "argv":[ws]
}, "create"), "workspace.create")
expect_ok(call("system", {
  "type":"yai.control.call.v1",
  "command_id":"yai.workspace.set",
  "target_plane":"runtime",
  "argv":[ws]
}, "set"), "workspace.set")

enroll = call(ws, {
  "type":"yai.control.call.v1",
  "command_id":"yai.source.enroll",
  "target_plane":"runtime",
  "workspace_id": ws,
  "source_label":"yd6-node-a",
  "owner_ref":"uds:///tmp/yai-owner.sock"
}, "enroll")
expect_ok(enroll, "source.enroll")
node_id = enroll.get("data", {}).get("source_node_id")
daemon_id = enroll.get("data", {}).get("daemon_instance_id")
trust_artifact_id = enroll.get("data", {}).get("owner_trust_artifact_id")
trust_artifact_token = enroll.get("data", {}).get("owner_trust_artifact_token")
if not node_id or not daemon_id:
    raise RuntimeError(f"enroll ids missing: {enroll}")
if not trust_artifact_id or not trust_artifact_token:
    raise RuntimeError(f"enroll trust bootstrap missing: {enroll}")

attach = call(ws, {
  "type":"yai.control.call.v1",
  "command_id":"yai.source.attach",
  "target_plane":"runtime",
  "workspace_id": ws,
  "source_node_id": node_id,
  "owner_trust_artifact_id": trust_artifact_id,
  "owner_trust_artifact_token": trust_artifact_token,
  "binding_scope":"workspace"
}, "attach")
expect_ok(attach, "source.attach")
binding_id = attach.get("data", {}).get("source_binding_id")
if not binding_id:
    raise RuntimeError(f"attach id missing: {attach}")

emit = call(ws, {
  "type":"yai.control.call.v1",
  "command_id":"yai.source.emit",
  "target_plane":"runtime",
  "workspace_id": ws,
  "source_node_id": node_id,
  "source_binding_id": binding_id,
  "owner_trust_artifact_id": trust_artifact_id,
  "owner_trust_artifact_token": trust_artifact_token,
  "idempotency_key":"yd6-emit-001",
  "source_assets":[
    {"type":"yai.source_asset.v1","source_asset_id":"sa-yd6-a","source_binding_id":binding_id,"locator":"file:///tmp/yd6-a.txt","asset_type":"file","provenance_fingerprint":"sha256:yd6a","observation_state":"observed"}
  ],
  "source_acquisition_events":[
    {"type":"yai.source_acquisition_event.v1","source_acquisition_event_id":"se-yd6-a","source_node_id":node_id,"source_binding_id":binding_id,"source_asset_id":"sa-yd6-a","event_type":"discovered","observed_at_epoch":1773190000,"idempotency_key":"yd6-emit-001","delivery_status":"received"}
  ],
  "source_evidence_candidates":[
    {"type":"yai.source_evidence_candidate.v1","source_evidence_candidate_id":"sc-yd6-a","source_acquisition_event_id":"se-yd6-a","candidate_type":"file_observation","derived_metadata_ref":"meta://yd6/a","owner_resolution_status":"pending"}
  ]
}, "emit")
expect_ok(emit, "source.emit")

status = call(ws, {
  "type":"yai.control.call.v1",
  "command_id":"yai.source.status",
  "target_plane":"runtime",
  "workspace_id": ws,
  "source_node_id": node_id,
  "daemon_instance_id": daemon_id,
  "owner_trust_artifact_id": trust_artifact_id,
  "owner_trust_artifact_token": trust_artifact_token,
  "health":"ready",
  "coverage_ref":"coverage://office/performance/kpi",
  "overlap_state":"overlap_possible",
  "backlog_queued":2,
  "backlog_retry_due":1,
  "backlog_failed":0
}, "status")
expect_ok(status, "source.status")

enroll_b = call(ws, {
  "type":"yai.control.call.v1",
  "command_id":"yai.source.enroll",
  "target_plane":"runtime",
  "workspace_id": ws,
  "source_label":"yd6-node-b",
  "owner_ref":"uds:///tmp/yai-owner.sock"
}, "enroll-b")
expect_ok(enroll_b, "source.enroll.b")
node_b = enroll_b.get("data", {}).get("source_node_id")
daemon_b = enroll_b.get("data", {}).get("daemon_instance_id")
trust_artifact_id_b = enroll_b.get("data", {}).get("owner_trust_artifact_id")
trust_artifact_token_b = enroll_b.get("data", {}).get("owner_trust_artifact_token")

attach_b = call(ws, {
  "type":"yai.control.call.v1",
  "command_id":"yai.source.attach",
  "target_plane":"runtime",
  "workspace_id": ws,
  "source_node_id": node_b,
  "owner_trust_artifact_id": trust_artifact_id_b,
  "owner_trust_artifact_token": trust_artifact_token_b,
  "binding_scope":"workspace",
  "coverage_ref":"coverage://office/performance/kpi",
  "overlap_state":"overlap_possible"
}, "attach-b")
expect_ok(attach_b, "source.attach.b")
binding_b = attach_b.get("data", {}).get("source_binding_id")

emit_dup = call(ws, {
  "type":"yai.control.call.v1",
  "command_id":"yai.source.emit",
  "target_plane":"runtime",
  "workspace_id": ws,
  "source_node_id": node_id,
  "source_binding_id": binding_id,
  "owner_trust_artifact_id": trust_artifact_id,
  "owner_trust_artifact_token": trust_artifact_token,
  "idempotency_key":"yd6-emit-001",
  "source_assets":[
    {"type":"yai.source_asset.v1","source_asset_id":"sa-yd6-a2","source_binding_id":binding_id,"locator":"file:///tmp/yd6-a.txt","asset_type":"file","provenance_fingerprint":"sha256:yd6a","observation_state":"observed"}
  ],
  "source_acquisition_events":[
    {"type":"yai.source_acquisition_event.v1","source_acquisition_event_id":"se-yd6-a2","source_node_id":node_id,"source_binding_id":binding_id,"source_asset_id":"sa-yd6-a2","event_type":"discovered","observed_at_epoch":1773190010,"idempotency_key":"yd6-emit-001","delivery_status":"received"}
  ]
}, "emit-dup")
expect_ok(emit_dup, "source.emit.dup")

emit_cross = call(ws, {
  "type":"yai.control.call.v1",
  "command_id":"yai.source.emit",
  "target_plane":"runtime",
  "workspace_id": ws,
  "source_node_id": node_b,
  "source_binding_id": binding_b,
  "owner_trust_artifact_id": trust_artifact_id_b,
  "owner_trust_artifact_token": trust_artifact_token_b,
  "idempotency_key":"yd6-emit-001",
  "source_assets":[
    {"type":"yai.source_asset.v1","source_asset_id":"sa-yd6-b1","source_binding_id":binding_b,"locator":"file:///tmp/yd6-b.txt","asset_type":"file","provenance_fingerprint":"sha256:yd6a","observation_state":"observed"}
  ],
  "source_acquisition_events":[
    {"type":"yai.source_acquisition_event.v1","source_acquisition_event_id":"se-yd6-b1","source_node_id":node_b,"source_binding_id":binding_b,"source_asset_id":"sa-yd6-b1","event_type":"discovered","observed_at_epoch":1773190020,"idempotency_key":"yd6-emit-001","delivery_status":"received"}
  ]
}, "emit-cross")
expect_ok(emit_cross, "source.emit.cross")

source_query = call(ws, {
  "type":"yai.control.call.v1",
  "command_id":"yai.workspace.query",
  "target_plane":"runtime",
  "argv":["source"]
}, "query-source")
expect_ok(source_query, "workspace.query source")
sq = source_query.get("data", {}).get("summary", {})
if sq.get("source_node_count", 0) < 1:
    raise RuntimeError(f"source summary missing source_node_count: {source_query}")
if sq.get("source_binding_count", 0) < 1:
    raise RuntimeError(f"source summary missing source_binding_count: {source_query}")
if sq.get("source_asset_count", 0) < 1:
    raise RuntimeError(f"source summary missing source_asset_count: {source_query}")
if sq.get("source_graph_node_count", 0) < 1:
    raise RuntimeError(f"source summary missing source_graph_node_count: {source_query}")
if sq.get("workspace_peer_membership_count", 0) < 1:
    raise RuntimeError(f"source summary missing workspace_peer_membership_count: {source_query}")
if sq.get("source_ingest_outcome_count", 0) < 3:
    raise RuntimeError(f"source summary missing source_ingest_outcome_count: {source_query}")
coord = source_query.get("data", {}).get("coordination", {})
if coord.get("peer_count", 0) < 1:
    raise RuntimeError(f"source coordination missing peer_count: {source_query}")
if not coord.get("scheduling_state"):
    raise RuntimeError(f"source coordination missing scheduling_state: {source_query}")
coverage = coord.get("coverage", {})
if coverage.get("scope_count", 0) < 1:
    raise RuntimeError(f"source coordination missing coverage scope_count: {source_query}")
if coverage.get("overlap", 0) < 1:
    raise RuntimeError(f"source coordination missing overlap count: {source_query}")
integrity = coord.get("integrity", {})
if integrity.get("replay_detected", 0) < 1:
    raise RuntimeError(f"source coordination missing replay_detected: {source_query}")
if integrity.get("overlap_detected", 0) < 1:
    raise RuntimeError(f"source coordination missing overlap_detected: {source_query}")

source_peer = call(ws, {
  "type":"yai.control.call.v1",
  "command_id":"yai.workspace.query",
  "target_plane":"runtime",
  "argv":["source.peer"]
}, "query-source-peer")
expect_ok(source_peer, "workspace.query source.peer")
peer_rows = source_peer.get("data", {}).get("rows", [])
if len(peer_rows) < 1:
    raise RuntimeError(f"source.peer missing rows: {source_peer}")
if not any(row.get("coverage_ref") == "coverage://office/performance/kpi" for row in peer_rows):
    raise RuntimeError(f"source.peer missing coverage_ref: {source_peer}")

source_coverage = call(ws, {
  "type":"yai.control.call.v1",
  "command_id":"yai.workspace.query",
  "target_plane":"runtime",
  "argv":["source.coverage"]
}, "query-source-coverage")
expect_ok(source_coverage, "workspace.query source.coverage")
coverage_summary = source_coverage.get("data", {}).get("coverage", {})
if coverage_summary.get("coverage_scope_count", 0) < 1:
    raise RuntimeError(f"source.coverage missing coverage_scope_count: {source_coverage}")
if coverage_summary.get("overlap_count", 0) < 1:
    raise RuntimeError(f"source.coverage missing overlap_count: {source_coverage}")

source_conflicts = call(ws, {
  "type":"yai.control.call.v1",
  "command_id":"yai.workspace.query",
  "target_plane":"runtime",
  "argv":["source.conflicts"]
}, "query-source-conflicts")
expect_ok(source_conflicts, "workspace.query source.conflicts")
conf_rows = source_conflicts.get("data", {}).get("rows", [])
if len(conf_rows) < 3:
    raise RuntimeError(f"source.conflicts missing rows: {source_conflicts}")

graph_ws = call(ws, {
  "type":"yai.control.call.v1",
  "command_id":"yai.workspace.graph.workspace",
  "target_plane":"runtime",
  "workspace_id": ws
}, "graph-workspace")
expect_ok(graph_ws, "graph.workspace")
gs = graph_ws.get("data", {}).get("summary", {})
if gs.get("source_graph_node_count", 0) < 1:
    raise RuntimeError(f"graph.workspace missing source_graph_node_count: {graph_ws}")
if gs.get("source_graph_edge_count", 0) < 1:
    raise RuntimeError(f"graph.workspace missing source_graph_edge_count: {graph_ws}")
PY

echo "source_plane_read_model: ok"
