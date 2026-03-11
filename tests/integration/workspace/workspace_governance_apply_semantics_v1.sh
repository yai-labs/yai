#!/usr/bin/env bash
set -euo pipefail

REPO="$(cd "$(dirname "${BASH_SOURCE[0]}")/../../.." && pwd)"
YAI="$REPO/build/bin/yai"
SOCK="${YAI_RUNTIME_INGRESS:-$HOME/.yai/run/control.sock}"
WS="ws_digital_outbound_prod"
ATTACH_OBJ="enterprise.ecohmedia.digital-outbound.review-gate"
BIND_FILE="$HOME/.yai/session/active_workspace.json"

make -C "$REPO" yai >/dev/null
make -C "$REPO" law-embed-sync >/dev/null

"$YAI" down >/dev/null 2>&1 || true
rm -f "$SOCK" >/dev/null 2>&1 || true
rm -f "$BIND_FILE" >/dev/null 2>&1 || true

RUNTIME_PID=""
cleanup() {
  if [[ -n "$RUNTIME_PID" ]] && kill -0 "$RUNTIME_PID" 2>/dev/null; then
    kill "$RUNTIME_PID" >/dev/null 2>&1 || true
    wait "$RUNTIME_PID" >/dev/null 2>&1 || true
  fi
  "$YAI" down --force >/dev/null 2>&1 || true
}
trap cleanup EXIT

(cd "$REPO" && "$YAI" up >/tmp/yai_workspace_governance_apply_runtime.log 2>&1) &
RUNTIME_PID=$!

for _ in $(seq 1 100); do
  [[ -S "$SOCK" ]] && break
  sleep 0.1
done
[[ -S "$SOCK" ]] || { echo "workspace_governance_apply_semantics_v1: FAIL (missing ingress socket)"; exit 1; }

python3 - "$SOCK" "$WS" "$ATTACH_OBJ" <<'PY'
import json
import socket
import struct
import sys

SOCK = sys.argv[1]
WS = sys.argv[2]
ATTACH_OBJ = sys.argv[3]

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
assert r["status"] == "ok", r

dry = call("system", "yai.workspace.policy_dry_run", [ATTACH_OBJ])
assert dry["status"] == "ok", dry
assert dry["data"]["eligibility_result"] in ("eligible", "eligible_with_warnings"), dry
assert dry["data"]["compatibility_result"] in ("dry_run_passed", "eligible_with_warnings"), dry
assert dry["data"]["ready_for_attach"] is True, dry

attach = call("system", "yai.workspace.policy_attach", [ATTACH_OBJ])
assert attach["status"] == "ok", attach
assert attach["data"]["attachment_state"] == "attached_active", attach
assert ATTACH_OBJ in attach["data"]["policy_attachments"], attach

activate = call("system", "yai.workspace.policy_activate", [ATTACH_OBJ])
assert activate["status"] == "ok", activate
assert activate["data"]["activation_state"] == "active", activate

effective = call("system", "yai.workspace.policy_effective")
assert effective["status"] == "ok", effective
assert ATTACH_OBJ in effective["data"]["operational_state"]["attached_governance_objects"], effective

detach = call("system", "yai.workspace.policy_detach", [ATTACH_OBJ])
assert detach["status"] == "ok", detach
assert ATTACH_OBJ not in detach["data"]["policy_attachments"], detach
assert detach["data"]["activation_state"] == "inactive", detach

call("system", "yai.workspace.unset")
PY

echo "workspace_governance_apply_semantics_v1: ok"
