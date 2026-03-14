#!/usr/bin/env bash
set -euo pipefail

REPO="$(cd "$(dirname "${BASH_SOURCE[0]}")/../../.." && pwd)"
YAI="$REPO/build/bin/yai"
SOCK="$HOME/.yai/run/control.sock"
BIND_FILE="$HOME/.yai/session/active_workspace.json"

if [[ ! -x "$YAI" ]]; then
  make -C "$REPO" yai >/dev/null
fi
make -C "$REPO" governance-sync >/dev/null

start_runtime() {
  local mode="${1:-normal}"
  env -u YAI_RUNTIME_INGRESS "$YAI" down >/dev/null 2>&1 || true
  rm -f "$SOCK" >/dev/null 2>&1 || true
  rm -f "$BIND_FILE" >/dev/null 2>&1 || true

  if [[ "$mode" == "partial" ]]; then
    (cd "$REPO" && YAI_ENFORCEMENT_RECORD_FORCE_PARTIAL=1 "$YAI" >/tmp/yai_workspace_dbfirst_partial.log 2>&1) &
  else
    (cd "$REPO" && env -u YAI_RUNTIME_INGRESS "$YAI" >/tmp/yai_workspace_dbfirst.log 2>&1) &
  fi
  RUNTIME_PID=$!

  for _ in $(seq 1 120); do
    [[ -S "$SOCK" ]] && break
    sleep 0.1
  done
  [[ -S "$SOCK" ]] || { echo "workspace_db_first_read_cutover: FAIL (missing ingress socket)"; exit 1; }
}

stop_runtime() {
  if [[ -n "${RUNTIME_PID:-}" ]] && kill -0 "$RUNTIME_PID" 2>/dev/null; then
    kill "$RUNTIME_PID" >/dev/null 2>&1 || true
    wait "$RUNTIME_PID" >/dev/null 2>&1 || true
  fi
  env -u YAI_RUNTIME_INGRESS "$YAI" down --force >/dev/null 2>&1 || true
  RUNTIME_PID=""
}

cleanup() {
  stop_runtime
}
trap cleanup EXIT

phase_check() {
  local ws="$1"
  local expect_fallback="$2"

  rm -rf "$HOME/.yai/run/$ws" >/dev/null 2>&1 || true

  python3 - "$SOCK" "$ws" "$expect_fallback" <<'PY'
import json
import socket
import struct
import sys
import time

SOCK = sys.argv[1]
WS = sys.argv[2]
EXPECT_FALLBACK = sys.argv[3].lower() == "true"

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


def assert_read_path(payload, label):
    enforcement_sensitive = {
        "yai.workspace.inspect",
        "yai.workspace.policy_effective",
        "yai.workspace.debug_resolution",
        "query:workspace",
        "query:enforcement",
    }
    rp = payload["read_path"]
    assert rp["mode"] == "db_first", rp
    assert rp["filesystem_primary"] is False, rp
    if EXPECT_FALLBACK:
        if label not in enforcement_sensitive:
            return
        assert rp["fallback_active"] is True, rp
        assert "enforcement_incomplete" in rp["fallback_reason"], rp
    else:
        if label == "query:governance" and rp["fallback_active"] is True:
            assert "missing_governance_" in rp["fallback_reason"], rp
        else:
            assert rp["db_first_ready"] is True, rp
            assert rp["fallback_active"] is False, rp


r = call(WS, "yai.workspace.create", [WS]); assert r["status"] == "ok", r
r = call("system", "yai.workspace.set", [WS]); assert r["status"] == "ok", r

for _ in range(20):
    r = call("system", "yai.workspace.domain_set", ["--family", "digital", "--specialization", "remote-publication"])
    if r["status"] == "ok":
        break
    time.sleep(0.1)
assert r["status"] == "ok", r

r = call(WS, "yai.workspace.run", ["digital.publish", "sink=external_untrusted", "contract=missing", "artifact=bundle-v1"])
assert r["status"] in ("ok", "error"), r

for command_id in ("yai.workspace.inspect", "yai.workspace.policy_effective", "yai.workspace.debug_resolution"):
    resp = call("system", command_id)
    assert resp["status"] == "ok", resp
    assert_read_path(resp["data"], command_id)

for family in ("workspace", "governance", "events", "evidence", "enforcement", "authority", "artifacts", "graph"):
    resp = call("system", "yai.workspace.query", [family])
    assert resp["status"] == "ok", resp
    data = resp["data"]
    assert data["query_family"] == family, data
    assert_read_path(data, f"query:{family}")

r = call("system", "yai.workspace.unset")
assert r["status"] == "ok", r
PY
}

WS_COMPLETE="ws_db_first_complete"
start_runtime normal
phase_check "$WS_COMPLETE" false
stop_runtime

WS_PARTIAL="ws_db_first_partial"
start_runtime partial
phase_check "$WS_PARTIAL" true
stop_runtime

echo "workspace_db_first_read_cutover: ok"
