#!/usr/bin/env bash
set -euo pipefail
source "$(dirname "$0")/_lib.sh"

python - <<"PY"
import json, os, hashlib

baseline_file = os.environ["BASELINE_FILE"]
baseline_id = os.environ["BASELINE_ID"]
evidence_dir = os.environ["EVIDENCE_DIR"]

with open(baseline_file, "rb") as f:
    blob = f.read()
bhash = hashlib.sha256(blob).hexdigest()

with open(os.path.join(evidence_dir, "baseline.json"), "w", encoding="utf-8") as f:
    json.dump({
        "baseline_id": baseline_id,
        "baseline_hash": bhash,
        "baseline_source": baseline_file,
    }, f, indent=2)
PY

python "$QT_DIR/metrics/parse_containment.py" \
  "$EVIDENCE_DIR/decision_records.jsonl" \
  "$EVIDENCE_DIR/containment_metrics.json"

cat > "$EVIDENCE_DIR/system_state.txt" <<EOF_STATE
run_id=$RUN_ID
domain_pack_id=$DOMAIN_PACK_ID
baseline_id=$BASELINE_ID
workload_id=$WORKLOAD_ID
attack_profile_id=$ATTACK_PROFILE_ID
runtime_mode=simulated
EOF_STATE

cat > "$EVIDENCE_DIR/EVIDENCE_INDEX.md" <<EOF_INDEX
# Evidence Index - $RUN_ID

- baseline.json
- timeline.jsonl
- decision_records.jsonl
- containment_metrics.json
- system_state.txt
EOF_INDEX

echo "evidence collected: $EVIDENCE_DIR"
