#!/usr/bin/env bash
set -euo pipefail
source "$(dirname "$0")/_lib.sh"

python3 - <<'PY'
import datetime
import hashlib
import json
import os
import socket
import struct
import threading
import time
import subprocess
from pathlib import Path

state_dir = Path(os.environ["STATE_DIR"])
evidence_dir = Path(os.environ["EVIDENCE_DIR"])
root_sock = os.environ["ROOT_SOCK"]
ws_id = os.environ["WS_ID"]
trace_id = os.environ["TRACE_ID"]
baseline_id = os.environ["BASELINE_ID"]
baseline_file = os.environ["BASELINE_FILE"]

mock_log = state_dir / "mock_server.log"
hits = {"count": 0}
stop_evt = threading.Event()
ready_evt = threading.Event()

def server_loop():
    srv = socket.socket(socket.AF_INET, socket.SOCK_STREAM)
    srv.setsockopt(socket.SOL_SOCKET, socket.SO_REUSEADDR, 1)
    srv.bind(("127.0.0.1", 8443))
    srv.listen(8)
    srv.settimeout(0.2)
    ready_evt.set()
    with mock_log.open("w", encoding="utf-8") as f:
        while not stop_evt.is_set():
            try:
                conn, _ = srv.accept()
            except TimeoutError:
                continue
            except OSError:
                break
            hits["count"] += 1
            f.write("hit\n")
            f.flush()
            try:
                conn.sendall(b"HTTP/1.1 200 OK\r\nContent-Length: 2\r\nConnection: close\r\n\r\nok")
            except Exception:
                pass
            try:
                conn.close()
            except Exception:
                pass
        f.write(f"count={hits['count']}\n")
    srv.close()

th = threading.Thread(target=server_loop, daemon=True)
th.start()
if not ready_evt.wait(timeout=3):
    raise SystemExit("failed to start local mock endpoint")

curl = subprocess.run([
    "curl", "-sS", "-o", "/dev/null", "-w", "%{http_code}",
    "-H", "Content-Type: application/json",
    "-d", "{\"resourceSpans\":[]}",
    "http://127.0.0.1:8443/v1/traces",
], capture_output=True, text=True)
local_target_reachable = (curl.returncode == 0 and curl.stdout.strip() == "200")
if not local_target_reachable:
    stop_evt.set(); th.join(timeout=2)
    raise SystemExit(f"otel precheck failed: rc={curl.returncode} out={curl.stdout!r} err={curl.stderr!r}")

precheck_hits = hits["count"]

def mk_env(command_id, payload_len):
    return struct.pack(
        "<II36s36sIHBxII",
        0x59414950,
        1,
        ws_id.encode("utf-8").ljust(36, b"\0"),
        trace_id.encode("utf-8").ljust(36, b"\0"),
        command_id,
        2,
        1,
        payload_len,
        0,
    )

def recv_exact(sock, n):
    data = b""
    while len(data) < n:
        chunk = sock.recv(n - len(data))
        if not chunk:
            raise RuntimeError("socket closed")
        data += chunk
    return data

handshake_payload = struct.pack("<II32s", 1, 0x0102, b"rt002-otel\0")
attack_payload = json.dumps({
    "target": {"host": "127.0.0.1", "port": 8443, "path": "/v1/traces"},
    "request": {"kind": "otel_export", "protocol": "otlp-http", "trial_id": "RT-002-otel-export-egress-v1"},
}).encode("utf-8")

with socket.socket(socket.AF_UNIX, socket.SOCK_STREAM) as cli:
    cli.connect(root_sock)
    cli.sendall(mk_env(0x0102, len(handshake_payload)))
    cli.sendall(handshake_payload)
    hs_env = recv_exact(cli, 96)
    hs_len = struct.unpack("<II36s36sIHBxII", hs_env)[7]
    hs_body = recv_exact(cli, hs_len) if hs_len else b""

    t0 = time.time()
    cli.sendall(mk_env(0x0301, len(attack_payload)))
    cli.sendall(attack_payload)
    resp_env_raw = recv_exact(cli, 96)
    resp_env = struct.unpack("<II36s36sIHBxII", resp_env_raw)
    resp_len = resp_env[7]
    resp_payload = recv_exact(cli, resp_len) if resp_len else b"{}"
    t1 = time.time()

time_to_contain_ms = int((t1 - t0) * 1000)

stop_evt.set(); th.join(timeout=2)
gated_hits = max(0, hits["count"] - precheck_hits)

with open(baseline_file, "rb") as f:
    baseline_hash = hashlib.sha256(f.read()).hexdigest()

resp_json = json.loads(resp_payload.decode("utf-8", errors="replace"))
decision = resp_json.get("decision", {})
enforcement = resp_json.get("enforcement", {})
metrics = resp_json.get("metrics", {})
outcome = decision.get("outcome", "deny")
reason = decision.get("reason_code", "EGRESS_DEST_NOT_CONTRACTED")
enf_result = enforcement.get("result", "blocked")
connect_established = bool(metrics.get("connect_established", False))
bytes_exfiltrated = int(metrics.get("bytes_exfiltrated", 0))

now = datetime.datetime.now(datetime.UTC).isoformat()

timeline = [
    {"ts": now, "step": "trigger", "event": "network.egress.connect", "status": "received", "mode": "live", "trial": "RT-002"},
    {"ts": now, "step": "precheck", "tool": "curl", "mode": "otlp-http", "local_target_reachable": local_target_reachable, "precheck_hits": precheck_hits},
    {"ts": now, "step": "decision", "outcome": outcome, "reason_code": reason},
    {"ts": now, "step": "enforcement", "result": enf_result, "connect_established": connect_established, "bytes_exfiltrated": bytes_exfiltrated, "gated_hits": gated_hits},
]
with (evidence_dir / "timeline.jsonl").open("w", encoding="utf-8") as f:
    for row in timeline:
        f.write(json.dumps(row) + "\n")

decision_record = {
    "timestamp": now,
    "ws_id": ws_id,
    "trace_id": trace_id,
    "event": {"type": "network.egress.connect", "source": "rt002-otel-live"},
    "principal": {"id": "principal-rt001", "role": "operator"},
    "target": {"dst": {"ip": "127.0.0.1", "port": 8443, "path": "/v1/traces"}},
    "decision": {"outcome": outcome, "reason_code": reason, "baseline_id": baseline_id, "baseline_hash": baseline_hash},
    "enforcement": {"result": enf_result},
    "metrics": {
        "time_to_contain_ms": time_to_contain_ms,
        "connect_established": connect_established,
        "bytes_exfiltrated": bytes_exfiltrated,
        "local_target_reachable": bool(local_target_reachable),
        "precheck_hits": precheck_hits,
        "gated_hits": gated_hits,
    },
}
(evidence_dir / "decision_records.jsonl").write_text(json.dumps(decision_record) + "\n", encoding="utf-8")
(state_dir / "attack_response.json").write_text(json.dumps({
    "handshake": {"payload_hex": hs_body.hex()},
    "response_envelope": {"magic": resp_env[0], "version": resp_env[1], "command_id": resp_env[4], "role": resp_env[5], "arming": resp_env[6], "payload_len": resp_env[7]},
    "response_payload": resp_json,
}, indent=2), encoding="utf-8")
PY

echo "trial executed (live): $ATTACK_PROFILE_ID"
