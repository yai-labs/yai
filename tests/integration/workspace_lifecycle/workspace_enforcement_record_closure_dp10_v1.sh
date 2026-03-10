#!/usr/bin/env bash
set -euo pipefail

REPO="$(cd "$(dirname "${BASH_SOURCE[0]}")/../../.." && pwd)"
YAI="$REPO/build/bin/yai"
SOCK="${YAI_RUNTIME_INGRESS:-$HOME/.yai/run/control.sock}"
BIND_FILE="$HOME/.yai/session/active_workspace.json"

if [[ ! -x "$YAI" ]]; then
  make -C "$REPO" yai >/dev/null
fi
make -C "$REPO" law-embed-sync >/dev/null

start_runtime() {
  local mode="${1:-normal}"
  "$YAI" down >/dev/null 2>&1 || true
  rm -f "$SOCK" >/dev/null 2>&1 || true
  rm -f "$BIND_FILE" >/dev/null 2>&1 || true

  if [[ "$mode" == "partial" ]]; then
    (cd "$REPO" && YAI_ENFORCEMENT_RECORD_FORCE_PARTIAL=1 "$YAI" >/tmp/yai_workspace_enforcement_dp10_partial.log 2>&1) &
  else
    (cd "$REPO" && "$YAI" >/tmp/yai_workspace_enforcement_dp10.log 2>&1) &
  fi
  RUNTIME_PID=$!

  for _ in $(seq 1 120); do
    [[ -S "$SOCK" ]] && break
    sleep 0.1
  done
  [[ -S "$SOCK" ]] || { echo "workspace_enforcement_record_closure_dp10_v1: FAIL (missing ingress socket)"; exit 1; }
}

stop_runtime() {
  if [[ -n "${RUNTIME_PID:-}" ]] && kill -0 "$RUNTIME_PID" 2>/dev/null; then
    kill "$RUNTIME_PID" >/dev/null 2>&1 || true
    wait "$RUNTIME_PID" >/dev/null 2>&1 || true
  fi
  "$YAI" down --force >/dev/null 2>&1 || true
  RUNTIME_PID=""
}

cleanup() {
  stop_runtime
}
trap cleanup EXIT

phase_check() {
  local ws="$1"
  local expect_status="$2"

  rm -rf "$HOME/.yai/run/$ws" >/dev/null 2>&1 || true

  python3 - "$SOCK" "$ws" "$HOME" "$expect_status" <<'PY'
import json
import os
import socket
import struct
import sys
import time

SOCK = sys.argv[1]
WS = sys.argv[2]
HOME = sys.argv[3]
EXPECT_STATUS = sys.argv[4]

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


r = call(WS, "yai.workspace.create", [WS]); assert r["status"] == "ok", r
r = call("system", "yai.workspace.set", [WS]); assert r["status"] == "ok", r

# Keep trying until family is accepted by embedded law pack load timing.
for _ in range(20):
    r = call("system", "yai.workspace.domain_set", ["--family", "digital", "--specialization", "remote-publication"])
    if r["status"] == "ok":
        break
    time.sleep(0.1)
assert r["status"] == "ok", r

r = call(WS, "yai.workspace.run", ["digital.publish", "sink=external_untrusted", "contract=missing", "artifact=bundle-v1"])
assert r["status"] in ("ok", "error"), r

q = call("system", "yai.workspace.query", ["enforcement"])
assert q["status"] == "ok", q
qd = q["data"]
assert qd["query_family"] == "enforcement", qd
assert qd["record"]["materialization_status"] == EXPECT_STATUS, qd
assert qd["record"]["outcome_ref"].startswith("enf-"), qd
assert qd["record"]["linkage_ref"].startswith("enl-"), qd

p = call("system", "yai.workspace.policy_effective")
assert p["status"] == "ok", p
pr = p["data"]["enforcement_record_set"]
assert pr["materialization_status"] == EXPECT_STATUS, pr

base = os.path.join(HOME, ".yai", "run", WS, "enforcement")
outcomes_log = os.path.join(base, "outcome-records.v1.ndjson")
linkage_log = os.path.join(base, "linkage-records.v1.ndjson")
index_path = os.path.join(base, "index.v1.json")
for path in (outcomes_log, linkage_log, index_path):
    assert os.path.exists(path), path

with open(index_path, "r", encoding="utf-8") as f:
    idx = json.load(f)
assert idx["type"] == "yai.enforcement.recordset.index.v1", idx
assert idx["workspace_id"] == WS, idx
assert idx["materialization_status"] == EXPECT_STATUS, idx

if EXPECT_STATUS == "complete":
    assert idx["missing_fields"] in ("", "none"), idx
else:
    assert idx["missing_fields"] not in ("", "none"), idx

for path, expected_type in (
    (outcomes_log, "yai.enforcement_outcome_record.v1"),
    (linkage_log, "yai.enforcement_linkage_record.v1"),
):
    with open(path, "r", encoding="utf-8") as f:
        lines = [ln.strip() for ln in f.readlines() if ln.strip()]
    assert lines, path
    last = json.loads(lines[-1])
    assert last["type"] == expected_type, last

r = call("system", "yai.workspace.unset")
assert r["status"] == "ok", r
PY
}

# Phase 1: normal writer path -> complete record set.
WS_COMPLETE="ws_enforcement_dp10_complete"
start_runtime normal
phase_check "$WS_COMPLETE" "complete"
stop_runtime

# Phase 2: forced partial path -> explicit incomplete semantics.
WS_PARTIAL="ws_enforcement_dp10_partial"
start_runtime partial
phase_check "$WS_PARTIAL" "incomplete"
stop_runtime

echo "workspace_enforcement_record_closure_dp10_v1: ok"
