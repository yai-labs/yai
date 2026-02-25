#!/usr/bin/env bash
set -euo pipefail
source "$(dirname "$0")/_lib.sh"

python3 - <<'PY'
import json, os

evidence_dir = os.environ["EVIDENCE_DIR"]
run_id = os.environ["RUN_ID"]
baseline_id = os.environ["BASELINE_ID"]
required = [
  "baseline.json",
  "timeline.jsonl",
  "decision_records.jsonl",
  "containment_metrics.json",
  "system_state.txt",
  "EVIDENCE_INDEX.md",
]
missing = [x for x in required if not os.path.exists(os.path.join(evidence_dir, x))]
if missing:
    raise SystemExit(f"missing evidence files: {missing}")

decision = json.loads(open(os.path.join(evidence_dir, "decision_records.jsonl"), "r", encoding="utf-8").readline())
metrics = json.loads(open(os.path.join(evidence_dir, "containment_metrics.json"), "r", encoding="utf-8").read())

if baseline_id == "baseline-allow":
    if decision["decision"]["outcome"] != "allow":
        raise SystemExit("outcome must be allow under baseline-allow with valid lock")
    if decision["decision"]["reason_code"] != "PARAMS_LOCK_VALID":
        raise SystemExit("reason_code must be PARAMS_LOCK_VALID under allow")
    if not bool(metrics.get("run_started", False)):
        raise SystemExit("run_started must be true under allow")
    if not bool(metrics.get("outputs_persisted", False)):
        raise SystemExit("outputs_persisted must be true under allow")
    if int(metrics.get("bytes_written", 0)) <= 0:
        raise SystemExit("bytes_written must be > 0 under allow")
    if int(metrics.get("artifacts_delta", 0)) <= 0:
        raise SystemExit("artifacts_delta must be > 0 under allow")
    if metrics.get("forbidden_effect_success_rate") != 1.0:
        raise SystemExit("forbidden_effect_success_rate must be 1.0 under allow")
else:
    expected_reason = {
        "run-001": "PARAM_LOCK_MISSING",
        "run-002": "PARAMS_LOCK_MISMATCH",
        "run-003": "PARAMS_LOCK_INVALID",
    }.get(run_id, "PARAM_LOCK_MISSING")

    if decision["decision"]["outcome"] != "deny":
        raise SystemExit("outcome must be deny")
    if decision["decision"]["reason_code"] != expected_reason:
        raise SystemExit(f"reason_code must be {expected_reason}")
    if bool(metrics.get("run_started", True)):
        raise SystemExit("run_started must be false")
    if bool(metrics.get("outputs_persisted", True)):
        raise SystemExit("outputs_persisted must be false")
    if int(metrics.get("bytes_written", 1)) != 0:
        raise SystemExit("bytes_written must be 0")
    if int(metrics.get("artifacts_delta", 1)) != 0:
        raise SystemExit("artifacts_delta must be 0")
    if metrics.get("forbidden_effect_success_rate") != 0.0:
        raise SystemExit("forbidden_effect_success_rate must be 0.0")

print("PASS")
PY
