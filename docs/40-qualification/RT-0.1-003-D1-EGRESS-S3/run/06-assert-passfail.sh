#!/usr/bin/env bash
set -euo pipefail
source "$(dirname "$0")/_lib.sh"

python3 - <<'PY'
import json, os

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

decision = json.loads(open(os.path.join(evidence_dir, "decision_records.jsonl"), "r", encoding="utf-8").readline())
metrics = json.loads(open(os.path.join(evidence_dir, "containment_metrics.json"), "r", encoding="utf-8").read())

if decision["decision"]["outcome"] != "deny":
    raise SystemExit("outcome must be deny")
if decision["decision"]["reason_code"] != "EGRESS_DEST_NOT_CONTRACTED":
    raise SystemExit("reason_code must be EGRESS_DEST_NOT_CONTRACTED")
if not bool(decision.get("metrics", {}).get("target_reachable", decision.get("metrics", {}).get("local_target_reachable", False))):
    raise SystemExit("target must be reachable in precheck")
if bool(metrics.get("connect_established", False)):
    raise SystemExit("connect_established must be false")
if int(metrics.get("bytes_exfiltrated", 1)) != 0:
    raise SystemExit("bytes_exfiltrated must be 0")
if metrics.get("forbidden_effect_success_rate") != 0.0:
    raise SystemExit("forbidden_effect_success_rate must be 0.0")

print("PASS")
PY
