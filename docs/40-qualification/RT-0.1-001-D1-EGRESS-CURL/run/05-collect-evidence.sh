#!/usr/bin/env bash
set -euo pipefail
source "$(dirname "$0")/_lib.sh"

python3 - <<'PY'
import json, os, hashlib
baseline_file = os.environ["BASELINE_FILE"]
baseline_id = os.environ["BASELINE_ID"]
evidence_dir = os.environ["EVIDENCE_DIR"]
with open(baseline_file, "rb") as f:
    bhash = hashlib.sha256(f.read()).hexdigest()
open(os.path.join(evidence_dir, "baseline.json"), "w", encoding="utf-8").write(json.dumps({
  "baseline_id": baseline_id,
  "baseline_hash": bhash,
  "baseline_source": baseline_file,
  "contract_ref": "policy.network_egress.allowlist"
}, indent=2))
PY

python3 "$RT_DIR/metrics/parse_containment.py" "$EVIDENCE_DIR/decision_records.jsonl" "$EVIDENCE_DIR/containment_metrics.json"

python3 - <<'PY'
import json, os, subprocess
from pathlib import Path

state_dir = Path(os.environ["STATE_DIR"])
evidence_dir = Path(os.environ["EVIDENCE_DIR"])
pids = json.loads((state_dir / "pids.json").read_text(encoding="utf-8")) if (state_dir / "pids.json").exists() else {}

lines = [
  f"run_id={os.environ['RUN_ID']}",
  f"domain_pack_id={os.environ['DOMAIN_PACK_ID']}",
  f"baseline_id={os.environ['BASELINE_ID']}",
  "runtime_mode=live",
  f"ws_id={os.environ['WS_ID']}",
  f"trace_id={os.environ['TRACE_ID']}",
  f"target_profile={os.environ.get('TARGET_PROFILE', 'local')}",
  f"target_url={os.environ.get('TARGET_URL', 'n/a')}",
  f"root_socket={os.environ['ROOT_SOCK']}",
  f"engine_socket={os.environ['ENGINE_SOCK']}",
  f"root_pid={pids.get('root_pid', 0)}",
  f"engine_pid={pids.get('engine_pid', 0)}",
  f"root_log={os.environ['ROOT_LOG']}",
  f"engine_log={os.environ['ENGINE_LOG']}",
]
sha = subprocess.check_output(["git", "rev-parse", "HEAD"], cwd=os.environ["REPO_ROOT"], text=True).strip()
lines.append(f"repo_sha={sha}")
(evidence_dir / "system_state.txt").write_text("\n".join(lines) + "\n", encoding="utf-8")

index = [
  f"# Evidence Index - {os.environ['RUN_ID']}",
  "",
  "- baseline.json",
  "- timeline.jsonl",
  "- decision_records.jsonl",
  "- containment_metrics.json",
  "- system_state.txt",
  "- .state/attack_response.json",
  "- ~/.yai/run/root/root.log",
  "- run/.state/<run>/engine.log",
]
(evidence_dir / "EVIDENCE_INDEX.md").write_text("\n".join(index) + "\n", encoding="utf-8")
PY

echo "evidence collected: $EVIDENCE_DIR"

python3 "$REPO_ROOT/docs/40-qualification/_shared/render_human_evidence.py" "$EVIDENCE_DIR" "${RT_DIR##*/}"
