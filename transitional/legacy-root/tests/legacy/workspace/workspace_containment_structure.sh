#!/usr/bin/env bash
set -euo pipefail

REPO="$(cd "$(dirname "${BASH_SOURCE[0]}")/../../.." && pwd)"
YAI="$REPO/build/bin/yai"
SOCK="${YAI_RUNTIME_INGRESS:-$HOME/.yai/run/control.sock}"
WSA="ws_struct_a"
WSB="ws_struct_b"

if [[ ! -x "$YAI" ]]; then
  make -C "$REPO" yai >/dev/null
fi

"$YAI" down >/dev/null 2>&1 || true
rm -f "$SOCK" >/dev/null 2>&1 || true

RUNTIME_PID=""
cleanup() {
  if [[ -n "$RUNTIME_PID" ]] && kill -0 "$RUNTIME_PID" 2>/dev/null; then
    kill "$RUNTIME_PID" >/dev/null 2>&1 || true
    wait "$RUNTIME_PID" >/dev/null 2>&1 || true
  fi
}
trap cleanup EXIT

(cd "$REPO" && "$YAI" >/tmp/yai_workspace_containment_structure_runtime.log 2>&1) &
RUNTIME_PID=$!
for _ in $(seq 1 50); do
  [[ -S "$SOCK" ]] && break
  sleep 0.1
done
[[ -S "$SOCK" ]] || { echo "workspace_containment_structure_v1: FAIL (missing ingress socket)"; exit 1; }

python3 - "$SOCK" "$WSA" "$WSB" <<'PY'
import json
import os
import socket
import struct
import sys
from pathlib import Path

SOCK = sys.argv[1]
WSA = sys.argv[2]
WSB = sys.argv[3]

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

for ws in (WSA, WSB):
    r = call(ws, "yai.workspace.create", [ws])
    assert r["status"] == "ok", r

r = call("system", "yai.workspace.set", [WSA])
assert r["status"] == "ok", r
r = call("system", "yai.workspace.domain_set", ["--family", "economic", "--specialization", "payments"])
assert r["status"] == "ok", r
r = call(WSA, "yai.workspace.run", ["payment.authorize", "provider=bank", "resource=money-transfer", "amount=20", "authority=owner"])
assert r["status"] in ("ok", "error"), r

# inspect must expose containment structure
r = call("system", "yai.workspace.inspect", [])
assert r["status"] == "ok", r
c = r["data"]["containment"]
assert c["layout"] == "v1", r
assert c["ready"] is True, r
assert c["state_surface"].endswith("/state/workspace-state.json"), r
assert c["traces_index"].endswith("/traces/index.json"), r
assert c["artifacts_index"].endswith("/artifacts/index.json"), r

# filesystem ownership separation A vs B
home = Path(os.path.expanduser("~"))
run_root = home / ".yai" / "run"
for ws in (WSA, WSB):
    ws_root = run_root / ws
    for rel in [
        "manifest.json",
        "metadata/binding.json",
        "state/workspace-state.json",
        "traces/index.json",
        "artifacts/index.json",
        "runtime/runtime-state.json",
    ]:
        assert (ws_root / rel).is_file(), (ws, rel)

# no cross-collision on paths
assert str((run_root / WSA / "traces/index.json")) != str((run_root / WSB / "traces/index.json"))
PY

"$REPO"/tools/dev/validate_workspace_structure.py "$WSA" "$WSB"

echo "workspace_containment_structure_v1: ok"
