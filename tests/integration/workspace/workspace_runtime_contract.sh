#!/usr/bin/env bash
set -euo pipefail

REPO="$(cd "$(dirname "${BASH_SOURCE[0]}")/../../.." && pwd)"
YAI="$REPO/build/bin/yai"
SOCK="$HOME/.yai/run/control.sock"
BIND_FILE="$HOME/.yai/session/active_workspace.json"

if [[ ! -x "$YAI" ]]; then
  make -C "$REPO" yai >/dev/null
fi

env -u YAI_RUNTIME_INGRESS "$YAI" down >/dev/null 2>&1 || true
rm -f "$SOCK" >/dev/null 2>&1 || true
rm -f "$BIND_FILE" >/dev/null 2>&1 || true

RUNTIME_PID=""
cleanup() {
  rm -f "$BIND_FILE" >/dev/null 2>&1 || true
  if [[ -n "$RUNTIME_PID" ]] && kill -0 "$RUNTIME_PID" 2>/dev/null; then
    kill "$RUNTIME_PID" >/dev/null 2>&1 || true
    wait "$RUNTIME_PID" >/dev/null 2>&1 || true
  fi
}
trap cleanup EXIT

(cd "$REPO" && env -u YAI_RUNTIME_INGRESS "$YAI" >/tmp/yai_workspace_runtime_up.log 2>&1) &
RUNTIME_PID=$!

for _ in $(seq 1 50); do
  if [[ -S "$SOCK" ]]; then
    break
  fi
  sleep 0.1
done

if [[ ! -S "$SOCK" ]]; then
  echo "workspace_runtime_contract_v1: FAIL (missing ingress socket $SOCK)"
  exit 1
fi

python3 - <<'PY'
import socket
import struct
import json
import os

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


def call(ws_id, command_id, argv=None):
    if argv is None:
        argv = []
    s = socket.socket(socket.AF_UNIX, socket.SOCK_STREAM)
    s.connect(SOCK)
    hs_payload = struct.pack(REQ_FMT, YAI_PROTOCOL_IDS_VERSION, 0, b"yai-test")
    s.sendall(build(YAI_CMD_HANDSHAKE, ws_id, hs_payload, "hs-1"))
    env = recv_exact(s, 96)
    _, _, _, _, cmd, _, _, _, plen, _ = struct.unpack(ENV_FMT, env)
    if cmd != YAI_CMD_HANDSHAKE:
      raise RuntimeError("bad handshake response")
    recv_exact(s, plen)

    payload = json.dumps({
        "type":"yai.control.call.v1",
        "command_id": command_id,
        "target_plane":"runtime",
        "argv": argv
    }).encode("utf-8")
    s.sendall(build(YAI_CMD_CONTROL_CALL, ws_id, payload, "call-1"))
    env = recv_exact(s, 96)
    _, _, _, _, cmd, _, _, _, plen, _ = struct.unpack(ENV_FMT, env)
    if cmd != YAI_CMD_CONTROL_CALL:
      raise RuntimeError("bad control response")
    body = recv_exact(s, plen).decode("utf-8")
    s.close()
    return json.loads(body)

def expect_ok(reply):
    if reply.get("status") != "ok":
      raise RuntimeError(f"expected ok reply, got {reply}")

def expect_cross_workspace_denied(reply):
    if reply.get("status") != "error":
      raise RuntimeError(f"expected error reply, got {reply}")
    if reply.get("code") != "BAD_ARGS":
      raise RuntimeError(f"expected BAD_ARGS, got {reply}")
    if reply.get("reason") != "cross_workspace_scope_denied":
      raise RuntimeError(f"expected cross_workspace_scope_denied, got {reply}")

ws1 = "ws_contract_01"
ws2 = "ws_contract_02"
ws3 = "ws_contract_ops"

# create three workspaces and activate first
expect_ok(call(ws1, "yai.workspace.create", [ws1]))
expect_ok(call(ws2, "yai.workspace.create", [ws2]))
expect_ok(call(ws3, "yai.workspace.create", [ws3]))
expect_ok(call("system", "yai.workspace.set", [ws1]))

# ping from active workspace works
r = call(ws1, "yai.runtime.ping", ["ping"])
expect_ok(r)
if r.get("code") not in ("OK", "REVIEW_REQUIRED"):
    raise RuntimeError(f"unexpected ping code for ws1: {r}")

# ping from non-active workspace must be denied
r = call(ws2, "yai.runtime.ping", ["ping"])
expect_cross_workspace_denied(r)

# switch and ensure context flip is enforced
expect_ok(call("system", "yai.workspace.open", [ws2]))
r = call(ws2, "yai.runtime.ping", ["ping"])
expect_ok(r)
if r.get("code") not in ("OK", "REVIEW_REQUIRED"):
    raise RuntimeError(f"unexpected ping code for ws2: {r}")
r = call(ws1, "yai.runtime.ping", ["ping"])
expect_cross_workspace_denied(r)

# status/inspect must expose canonical family model and no legacy projection
st = call("system", "yai.workspace.status")
expect_ok(st)
if st.get("data", {}).get("binding_status") != "active":
    raise RuntimeError(f"status did not report active binding: {st}")
caps = st.get("data", {}).get("runtime_capabilities", {})
if not isinstance(caps, dict):
    raise RuntimeError(f"runtime_capabilities not an object: {st}")
for fam in ("exec", "data", "graph", "knowledge"):
    if fam not in caps:
        raise RuntimeError(f"missing capability family {fam}: {st}")

ins = call("system", "yai.workspace.inspect")
expect_ok(ins)
data = ins.get("data", {})
if data.get("identity", {}).get("workspace_id") != ws2:
    raise RuntimeError(f"inspect did not follow active workspace switch: {ins}")
if "brain_persistence" in data:
    raise RuntimeError(f"legacy brain_persistence projection still exposed: {ins}")
if "graph_persistence" not in data or "knowledge_transient_persistence" not in data:
    raise RuntimeError(f"canonical persistence surfaces missing: {ins}")
read_path = data.get("read_path", {})
if read_path.get("mode") != "db_first":
    raise RuntimeError(f"read_path mode is not db_first: {ins}")
recovery = data.get("runtime_capabilities", {}).get("recovery", {})
if recovery.get("tracked") is not True:
    raise RuntimeError(f"recovery tracking not exposed: {ins}")
if recovery.get("state") not in ("fresh", "recovered", "restored"):
    raise RuntimeError(f"unexpected recovery state: {ins}")
PY

for ws in ws_contract_01 ws_contract_02 ws_contract_ops; do
  if [[ ! -d "$HOME/.yai/run/$ws" ]]; then
    echo "workspace_runtime_contract_v1: FAIL (missing run dir for $ws)"
    exit 1
  fi
  for rel in data graph knowledge transient; do
    if [[ ! -d "$HOME/.yai/run/data/$ws/$rel" ]]; then
      echo "workspace_runtime_contract_v1: FAIL (missing runtime data dir $HOME/.yai/run/data/$ws/$rel)"
      exit 1
    fi
  done
done

echo "workspace_runtime_contract_v1: ok"
