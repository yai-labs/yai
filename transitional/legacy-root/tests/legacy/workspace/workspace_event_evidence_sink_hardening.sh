#!/usr/bin/env bash
set -euo pipefail

REPO="$(cd "$(dirname "${BASH_SOURCE[0]}")/../../.." && pwd)"
YAI="$REPO/build/bin/yai"
SOCK="$HOME/.yai/run/control.sock"
WS="ws_event_evidence_sink_v1"
BIND_FILE="$HOME/.yai/session/active_workspace.json"

make -C "$REPO" yai >/dev/null
make -C "$REPO" governance-sync >/dev/null

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

(cd "$REPO" && env -u YAI_RUNTIME_INGRESS "$YAI" >/tmp/yai_workspace_event_evidence_sink_runtime.log 2>&1) &
RUNTIME_PID=$!

for _ in $(seq 1 120); do
  [[ -S "$SOCK" ]] && break
  sleep 0.1
done
[[ -S "$SOCK" ]] || { echo "workspace_event_evidence_sink_hardening_v1: FAIL (missing ingress socket)"; exit 1; }

python3 - "$SOCK" "$WS" "$HOME" <<'PY'
import json
import os
import socket
import struct
import sys

SOCK = sys.argv[1]
WS = sys.argv[2]
HOME = sys.argv[3]

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


r = call(WS, "yai.workspace.create", [WS])
assert r["status"] == "ok", r
r = call("system", "yai.workspace.set", [WS])
assert r["status"] == "ok", r
r = call("system", "yai.workspace.domain_set", ["--family", "digital", "--specialization", "remote-publication"])
if not (r["status"] == "ok" or (r["status"] == "error" and r.get("reason") == "family_not_found")):
    raise AssertionError(r)

r = call(WS, "yai.workspace.run", ["digital.publish", "sink=external_untrusted", "contract=missing", "artifact=bundle-v1"])
assert r["status"] in ("ok", "error"), r

p = call("system", "yai.workspace.policy_effective")
assert p["status"] == "ok", p
sink = p["data"].get("event_evidence_sink")
assert isinstance(sink, dict), p
assert sink.get("last_event_ref", "").startswith("evt-"), sink
assert sink.get("last_decision_ref", "").startswith("dec-"), sink
assert sink.get("last_evidence_ref", "").startswith("evd-"), sink

base = os.path.join(HOME, ".yai", "run", WS, "events")
events_log = os.path.join(base, "runtime-events.v1.ndjson")
decision_log = os.path.join(base, "decision-records.v1.ndjson")
evidence_log = os.path.join(base, "evidence-records.v1.ndjson")
index_path = os.path.join(base, "index.v1.json")

for path in (events_log, decision_log, evidence_log, index_path):
    assert os.path.exists(path), path

with open(index_path, "r", encoding="utf-8") as f:
    idx = json.load(f)

assert idx["type"] == "yai.event_evidence.index.v1", idx
assert idx["workspace_id"] == WS, idx
assert idx["last_event_ref"].startswith("evt-"), idx
assert idx["last_decision_ref"].startswith("dec-"), idx
assert idx["last_evidence_ref"].startswith("evd-"), idx

def last_ndjson(path):
    with open(path, "r", encoding="utf-8") as f:
        lines = [ln.strip() for ln in f.readlines() if ln.strip()]
    assert lines, path
    return json.loads(lines[-1])

ev = last_ndjson(events_log)
dec = last_ndjson(decision_log)
evi = last_ndjson(evidence_log)

assert ev["type"] == "yai.runtime_event.v1", ev
assert ev["event_id"] == idx["last_event_ref"], (ev, idx)
assert ev["decision_ref"] == idx["last_decision_ref"], (ev, idx)
assert ev["evidence_ref"] == idx["last_evidence_ref"], (ev, idx)

assert dec["type"] == "yai.decision_record.v1", dec
assert dec["decision_id"] == idx["last_decision_ref"], (dec, idx)

assert evi["type"] == "yai.evidence_record.v1", evi
assert evi["decision_ref"] == idx["last_decision_ref"], (evi, idx)
assert evi["trace_ref"] == idx["last_trace_ref"], (evi, idx)

r = call("system", "yai.workspace.unset")
assert r["status"] == "ok", r
PY

echo "workspace_event_evidence_sink_hardening_v1: ok"
