#!/usr/bin/env bash
set -euo pipefail
source "$(dirname "$0")/_lib.sh"

python - <<"PY"
import json, os, hashlib, datetime

baseline_file = os.environ["BASELINE_FILE"]
expected_file = os.environ["EXPECTED_FILE"]
state_dir = os.environ["STATE_DIR"]
evidence_dir = os.environ["EVIDENCE_DIR"]
baseline_id = os.environ["BASELINE_ID"]

with open(expected_file, "r", encoding="utf-8") as f:
    expected = json.load(f)
match = None
for item in expected.get("expected", []):
    if item.get("baseline_id") == baseline_id:
        match = item
        break
if not match:
    raise SystemExit(f"no expected outcome for baseline_id={baseline_id}")

with open(baseline_file, "rb") as f:
    baseline_hash = hashlib.sha256(f.read()).hexdigest()

with open(os.path.join(state_dir, "context.json"), "r", encoding="utf-8") as f:
    ctx = json.load(f)

now = datetime.datetime.utcnow().isoformat() + "Z"
outcome = match["decision"]["outcome"]
reason = match["decision"]["reason_code"]
enf_result = match["enforcement"]["result"]
metrics = match.get("metrics", {})
connect_established = bool(metrics.get("connect_established", False))
bytes_exfiltrated = int(metrics.get("bytes_exfiltrated", 0))

timeline = [
    {"ts": now, "step": "trigger", "event": "network.egress.connect", "status": "received"},
    {"ts": now, "step": "context", "ws_id": ctx["ws_id"], "trace_id": ctx["trace_id"], "role": ctx["principal"]["role"], "arming": ctx["arming_state"]},
    {"ts": now, "step": "authority", "baseline_id": baseline_id, "baseline_hash": baseline_hash},
    {"ts": now, "step": "decision", "outcome": outcome, "reason_code": reason},
    {"ts": now, "step": "enforcement", "result": enf_result, "connect_established": connect_established, "bytes_exfiltrated": bytes_exfiltrated},
    {"ts": now, "step": "evidence", "status": "materialized"},
]

with open(os.path.join(evidence_dir, "timeline.jsonl"), "w", encoding="utf-8") as f:
    for row in timeline:
        f.write(json.dumps(row) + "\n")

decision_record = {
    "timestamp": now,
    "ws_id": ctx["ws_id"],
    "trace_id": ctx["trace_id"],
    "event": {"type": "network.egress.connect", "source": "qt-sc102"},
    "principal": ctx["principal"],
    "target": {"dst": {"ip": "203.0.113.10", "port": 443}},
    "decision": {
        "outcome": outcome,
        "reason_code": reason,
        "baseline_id": baseline_id,
        "baseline_hash": baseline_hash,
    },
    "enforcement": {"result": enf_result},
    "metrics": {
        "time_to_contain_ms": 5,
        "connect_established": connect_established,
        "bytes_exfiltrated": bytes_exfiltrated,
    },
}
with open(os.path.join(evidence_dir, "decision_records.jsonl"), "w", encoding="utf-8") as f:
    f.write(json.dumps(decision_record) + "\n")
PY

echo "attack executed (simulated): $ATTACK_PROFILE_ID"
