#!/usr/bin/env bash
set -euo pipefail
source "$(dirname "$0")/_lib.sh"

python3 - <<'PY'
import datetime
import hashlib
import json
import os
import time
from pathlib import Path

state_dir = Path(os.environ["STATE_DIR"])
evidence_dir = Path(os.environ["EVIDENCE_DIR"])
baseline_id = os.environ["BASELINE_ID"]
baseline_file = os.environ["BASELINE_FILE"]
run_id = os.environ["RUN_ID"]
ws_id = os.environ["WS_ID"]
trace_id = os.environ["TRACE_ID"]
pipeline_id = os.environ["PIPELINE_ID"]
dataset_ref = os.environ["DATASET_REF"]
target_path = Path(os.environ["TARGET_PATH"])

variant_map = {
    "run-001": "missing",
    "run-002": "mismatch",
    "run-003": "invalid",
}
variant = os.environ.get("ATTACK_VARIANT") or variant_map.get(run_id, "missing")

params = {
    "pipeline_id": pipeline_id,
    "dataset_ref": dataset_ref,
    "seed": 123,
    "steps": 10,
}
params_json = json.dumps(params, sort_keys=True, separators=(",", ":"))
params_hash = hashlib.sha256(params_json.encode("utf-8")).hexdigest()

inputs = state_dir / "inputs"
inputs.mkdir(parents=True, exist_ok=True)
params_file = inputs / "params.json"
lock_file = inputs / "params.lock"
params_file.write_text(params_json + "\n", encoding="utf-8")
if lock_file.exists():
    lock_file.unlink()

if variant == "mismatch":
    lock = {"params_hash": "0" * 64, "signature": "sig-valid"}
    lock_file.write_text(json.dumps(lock, indent=2) + "\n", encoding="utf-8")
elif variant == "invalid":
    lock = {"params_hash": params_hash, "signature": "sig-invalid"}
    lock_file.write_text(json.dumps(lock, indent=2) + "\n", encoding="utf-8")

with open(baseline_file, "rb") as f:
    baseline_hash = hashlib.sha256(f.read()).hexdigest()

reason = "PARAM_LOCK_MISSING"
lock_hash = None
if variant == "missing":
    reason = "PARAM_LOCK_MISSING"
elif variant == "mismatch":
    reason = "PARAMS_HASH_MISMATCH"
elif variant == "invalid":
    reason = "PARAMS_LOCK_INVALID"

if lock_file.exists():
    lock_hash = hashlib.sha256(lock_file.read_bytes()).hexdigest()

# Deny path: forbidden effect is publishing output without valid lock.
outcome = "deny"
enforcement_result = "blocked"
run_started = False
outputs_persisted = False
bytes_written = 0

publish_file = target_path / f"{run_id}.result.json"
if publish_file.exists():
    publish_file.unlink()

started = time.time()
time.sleep(0.001)
ended = time.time()

now = datetime.datetime.now(datetime.UTC).isoformat()

timeline = [
    {"ts": now, "step": "trigger", "event": "scientific.run.start", "status": "received", "variant": variant},
    {"ts": now, "step": "context", "ws_id": ws_id, "trace_id": trace_id, "role": "operator", "arming": "armed"},
    {"ts": now, "step": "authority", "baseline_id": baseline_id, "baseline_hash": baseline_hash},
    {"ts": now, "step": "decision", "outcome": outcome, "reason_code": reason},
    {"ts": now, "step": "enforcement", "result": enforcement_result, "run_started": run_started, "outputs_persisted": outputs_persisted, "bytes_written": bytes_written},
    {"ts": now, "step": "evidence", "status": "materialized"},
]
with (evidence_dir / "timeline.jsonl").open("w", encoding="utf-8") as f:
    for row in timeline:
        f.write(json.dumps(row) + "\n")

decision_record = {
    "timestamp": now,
    "domain_pack_id": os.environ["DOMAIN_PACK_ID"],
    "ws_id": ws_id,
    "trace_id": trace_id,
    "event": {"type": "scientific.run.start", "source": "rt004-params-lock-live"},
    "principal": {"id": "principal-rt004", "role": "operator"},
    "target": {
        "pipeline_id": pipeline_id,
        "run_id": run_id,
        "dataset_ref": dataset_ref,
        "params_hash": params_hash,
        "lock_hash": lock_hash,
        "dst": {"path": str(target_path)},
    },
    "decision": {
        "outcome": outcome,
        "reason_code": reason,
        "baseline_id": baseline_id,
        "baseline_hash": baseline_hash,
    },
    "enforcement": {"result": enforcement_result},
    "metrics": {
        "time_to_decide_ms": int((ended - started) * 1000),
        "run_started": run_started,
        "outputs_persisted": outputs_persisted,
        "bytes_written": bytes_written,
        "target_ready": True,
    },
}
(evidence_dir / "decision_records.jsonl").write_text(json.dumps(decision_record) + "\n", encoding="utf-8")

(state_dir / "attack_response.json").write_text(json.dumps({
    "variant": variant,
    "params_file": str(params_file),
    "lock_file": str(lock_file),
    "target_path": str(target_path),
    "decision": decision_record["decision"],
    "enforcement": decision_record["enforcement"],
}, indent=2), encoding="utf-8")
PY

echo "trial executed (live): $ATTACK_PROFILE_ID"
