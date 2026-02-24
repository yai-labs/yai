#!/usr/bin/env bash
set -euo pipefail
source "$(dirname "$0")/_lib.sh"

python - <<"PY"
import json, os, sys

expected_file = os.environ["EXPECTED_FILE"]
baseline_id = os.environ["BASELINE_ID"]
evidence_dir = os.environ["EVIDENCE_DIR"]

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

with open(expected_file, "r", encoding="utf-8") as f:
    expected = json.load(f)
exp = None
for item in expected.get("expected", []):
    if item.get("baseline_id") == baseline_id:
        exp = item
        break
if exp is None:
    raise SystemExit(f"missing expected baseline entry: {baseline_id}")

with open(os.path.join(evidence_dir, "decision_records.jsonl"), "r", encoding="utf-8") as f:
    decision = json.loads(f.readline())
with open(os.path.join(evidence_dir, "containment_metrics.json"), "r", encoding="utf-8") as f:
    metrics = json.load(f)

act_outcome = decision["decision"]["outcome"]
exp_outcome = exp["decision"]["outcome"]
if act_outcome != exp_outcome:
    raise SystemExit(f"outcome mismatch: expected={exp_outcome} actual={act_outcome}")

act_reason = decision["decision"]["reason_code"]
exp_reason = exp["decision"]["reason_code"]
if act_reason != exp_reason:
    raise SystemExit(f"reason mismatch: expected={exp_reason} actual={act_reason}")

if metrics.get("forbidden_effect_success_rate") != 0.0:
    raise SystemExit("forbidden effect success rate must be 0.0")
if bool(metrics.get("connect_established", False)):
    raise SystemExit("connect_established must be false")
if int(metrics.get("bytes_exfiltrated", 1)) != 0:
    raise SystemExit("bytes_exfiltrated must be 0")

print("PASS")
PY
