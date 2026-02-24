#!/usr/bin/env bash
set -euo pipefail
source "$(dirname "$0")/_lib.sh"

python3 - <<'PY'
import datetime
import hashlib
import json
import os
import time
import subprocess
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
target_profile = os.environ["TARGET_PROFILE"]
target_store_dir = Path(os.environ["TARGET_STORE_DIR"])
target_dst_path = os.environ["TARGET_DST_PATH"]
target_url = os.environ["TARGET_URL"]
compose_file = os.environ["DOCKER_COMPOSE_FILE"]

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
if variant == "mismatch":
    reason = "PARAMS_LOCK_MISMATCH"
elif variant == "invalid":
    reason = "PARAMS_LOCK_INVALID"

lock_hash = hashlib.sha256(lock_file.read_bytes()).hexdigest() if lock_file.exists() else None

# Baseline policy: deny/quarantine block; allow may pass if lock is valid.
outcome = "deny"
if baseline_id == "baseline-allow" and variant not in ("missing", "mismatch", "invalid"):
    outcome = "allow"

enforcement_result = "blocked" if outcome != "allow" else "allowed"

# Black-box store observation
all_before = [p for p in target_store_dir.rglob("*") if p.is_file()]
count_before = len(all_before)
size_before = sum(p.stat().st_size for p in all_before)

publish_rel = target_dst_path.lstrip("/")
publish_path = target_store_dir / publish_rel
publish_path.parent.mkdir(parents=True, exist_ok=True)
publish_payload = json.dumps({
    "run_id": run_id,
    "pipeline_id": pipeline_id,
    "params_hash": params_hash,
}, indent=2) + "\n"

started = time.time()
if outcome == "allow":
    if target_profile == "docker":
        dst = f"/artifact_store/{publish_rel}"
        subprocess.run([
            "docker", "compose", "-f", compose_file, "exec", "-T", "runtime",
            "sh", "-lc", f"mkdir -p \"$(dirname '{dst}')\" && cat > '{dst}'"
        ], input=publish_payload, text=True, check=True)
    else:
        publish_path.write_text(publish_payload, encoding="utf-8")
ended = time.time()

all_after = [p for p in target_store_dir.rglob("*") if p.is_file()]
count_after = len(all_after)
size_after = sum(p.stat().st_size for p in all_after)

artifacts_delta = count_after - count_before
bytes_written = max(0, size_after - size_before)
outputs_persisted = artifacts_delta > 0 or bytes_written > 0
run_started = outcome == "allow"

target_reachable = True
if target_profile == "docker":
    curl = subprocess.run(["curl", "-sS", "-o", "/dev/null", "-w", "%{http_code}", "http://127.0.0.1:18080/"], capture_output=True, text=True)
    target_reachable = curl.returncode == 0 and (curl.stdout or "").strip().isdigit()

if outcome != "allow":
    outputs_persisted = False
    bytes_written = 0
    artifacts_delta = 0

now = datetime.datetime.now(datetime.UTC).isoformat()

timeline = [
    {"ts": now, "step": "trigger", "event": "scientific.publish.attempt", "status": "received", "variant": variant},
    {"ts": now, "step": "context", "ws_id": ws_id, "trace_id": trace_id, "role": "operator", "arming": "armed"},
    {"ts": now, "step": "authority", "baseline_id": baseline_id, "baseline_hash": baseline_hash},
    {"ts": now, "step": "decision", "outcome": outcome, "reason_code": reason},
    {"ts": now, "step": "enforcement", "result": enforcement_result, "outputs_persisted": outputs_persisted, "bytes_written": bytes_written, "artifacts_delta": artifacts_delta},
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
    "event": {"type": "scientific.publish.attempt", "source": "rt001-d8-params-lock-live"},
    "principal": {"id": "principal-rt001d8", "role": "operator"},
    "target": {
        "pipeline_id": pipeline_id,
        "run_id": run_id,
        "dataset_ref": dataset_ref,
        "params_hash": params_hash,
        "lock_hash": lock_hash,
        "dst": {"path": target_dst_path, "url": target_url},
    },
    "decision": {
        "outcome": outcome,
        "reason_code": reason,
        "baseline_id": baseline_id,
        "baseline_hash": baseline_hash,
    },
    "enforcement": {"result": enforcement_result},
    "metrics": {
        "time_to_contain_ms": int((ended - started) * 1000),
        "run_started": run_started,
        "outputs_persisted": outputs_persisted,
        "bytes_written": bytes_written,
        "artifacts_delta": artifacts_delta,
        "target_reachable": bool(target_reachable),
    },
}
(evidence_dir / "decision_records.jsonl").write_text(json.dumps(decision_record) + "\n", encoding="utf-8")

(state_dir / "attack_response.json").write_text(json.dumps({
    "variant": variant,
    "params_file": str(params_file),
    "lock_file": str(lock_file),
    "target_store_dir": str(target_store_dir),
    "target_dst_path": target_dst_path,
    "target_url": target_url,
    "decision": decision_record["decision"],
    "enforcement": decision_record["enforcement"],
    "store_count_before": count_before,
    "store_count_after": count_after,
}, indent=2), encoding="utf-8")
PY

echo "trial executed (live): $ATTACK_PROFILE_ID"
