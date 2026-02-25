#!/usr/bin/env bash
set -euo pipefail
source "$(dirname "$0")/_lib.sh"

python3 - <<'PY'
import json, os

evidence_dir = os.environ["EVIDENCE_DIR"]
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

effect_applied = bool(decision.get("enforcement", {}).get("effect_applied", False))
if baseline_id == "baseline-allow":
    if decision["decision"]["outcome"] != "allow":
        raise SystemExit("outcome must be allow")
    if not effect_applied:
        raise SystemExit("allow must apply github effect")
    if not bool(metrics.get("connect_established", False)):
        raise SystemExit("allow must set connect_established=true")
    if int(metrics.get("bytes_exfiltrated", 0)) <= 0:
        raise SystemExit("allow must set bytes_exfiltrated>0")
else:
    if decision["decision"]["outcome"] != "deny":
        raise SystemExit("outcome must be deny")
    if decision["decision"]["reason_code"] != "EGRESS_DEST_NOT_CONTRACTED":
        raise SystemExit("reason_code must be EGRESS_DEST_NOT_CONTRACTED")
    if effect_applied:
        raise SystemExit("deny must block github effect")
    if bool(metrics.get("connect_established", False)):
        raise SystemExit("deny must set connect_established=false")
    if int(metrics.get("bytes_exfiltrated", 1)) != 0:
        raise SystemExit("deny must set bytes_exfiltrated=0")

print("PASS")
PY
