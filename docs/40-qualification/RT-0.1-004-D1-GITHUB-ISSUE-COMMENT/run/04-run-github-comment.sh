#!/usr/bin/env bash
set -euo pipefail
source "$(dirname "$0")/_lib.sh"

python3 - <<'PY'
import datetime
import hashlib
import json
import os
import subprocess
import time
from pathlib import Path

state_dir = Path(os.environ["STATE_DIR"])
evidence_dir = Path(os.environ["EVIDENCE_DIR"])
baseline_id = os.environ["BASELINE_ID"]
baseline_file = os.environ["BASELINE_FILE"]
run_id = os.environ["RUN_ID"]
ws_id = os.environ["WS_ID"]
trace_id = os.environ["TRACE_ID"]
repo = os.environ["GH_REPO"]
issue = os.environ["GH_ISSUE_NUMBER"]
prefix = os.environ["GH_COMMENT_PREFIX"]
cleanup = os.environ.get("GH_COMMENT_CLEANUP", "0") == "1"

state_dir.mkdir(parents=True, exist_ok=True)
evidence_dir.mkdir(parents=True, exist_ok=True)

with open(baseline_file, "rb") as f:
    baseline_hash = hashlib.sha256(f.read()).hexdigest()

comment_body = f"{prefix} run={run_id} ws={ws_id} trace={trace_id} ts={datetime.datetime.now(datetime.UTC).isoformat()}"

start = time.time()
outcome = "deny"
reason = "EGRESS_DEST_NOT_CONTRACTED"
enforcement = "blocked"
connect_established = False
bytes_exfiltrated = 0
effect_applied = False
comment_id = None
comment_url = None

if baseline_id == "baseline-allow":
    cmd = [
        "gh", "api",
        f"repos/{repo}/issues/{issue}/comments",
        "-f", f"body={comment_body}",
    ]
    cp = subprocess.run(cmd, capture_output=True, text=True)
    if cp.returncode != 0:
        raise SystemExit(f"github comment create failed: {cp.stderr.strip()}")
    payload = json.loads(cp.stdout)
    comment_id = payload.get("id")
    comment_url = payload.get("html_url")
    outcome = "allow"
    reason = "EGRESS_DEST_CONTRACTED"
    enforcement = "allowed"
    connect_established = True
    bytes_exfiltrated = len(comment_body.encode("utf-8"))
    effect_applied = True

    if cleanup and comment_id:
        subprocess.run(["gh", "api", "-X", "DELETE", f"repos/{repo}/issues/comments/{comment_id}"], check=True)

end = time.time()
now = datetime.datetime.now(datetime.UTC).isoformat()

timeline = [
    {"ts": now, "step": "trigger", "event": "github.issue.comment.create", "status": "received"},
    {"ts": now, "step": "context", "ws_id": ws_id, "trace_id": trace_id, "role": "operator", "arming": "armed"},
    {"ts": now, "step": "authority", "baseline_id": baseline_id, "baseline_hash": baseline_hash},
    {"ts": now, "step": "decision", "outcome": outcome, "reason_code": reason},
    {
        "ts": now,
        "step": "enforcement",
        "result": enforcement,
        "effect_applied": effect_applied,
        "comment_id": comment_id,
        "comment_url": comment_url,
    },
    {"ts": now, "step": "evidence", "status": "materialized"},
]
with (evidence_dir / "timeline.jsonl").open("w", encoding="utf-8") as f:
    for row in timeline:
        f.write(json.dumps(row) + "\n")

decision_record = {
    "timestamp": now,
    "ws_id": ws_id,
    "trace_id": trace_id,
    "event": {"type": "github.issue.comment.create", "source": "rt004-github-live"},
    "principal": {"id": "principal-rt004", "role": "operator"},
    "target": {
        "provider": "github",
        "repo": repo,
        "issue_number": int(issue),
        "dst": {
            "ip": "api.github.com",
            "port": 443,
            "path": f"/repos/{repo}/issues/{issue}/comments",
            "url": f"https://api.github.com/repos/{repo}/issues/{issue}/comments"
        },
    },
    "decision": {
        "outcome": outcome,
        "reason_code": reason,
        "baseline_id": baseline_id,
        "baseline_hash": baseline_hash,
    },
    "enforcement": {
        "result": enforcement,
        "effect_applied": effect_applied,
        "comment_id": comment_id,
        "comment_url": comment_url,
    },
    "metrics": {
        "time_to_contain_ms": int((end - start) * 1000),
        "connect_established": connect_established,
        "bytes_exfiltrated": bytes_exfiltrated,
        "target_reachable": True,
        "local_target_reachable": False,
        "precheck_hits": 0,
        "gated_hits": 0,
        "effect_applied": effect_applied,
    },
}
(evidence_dir / "decision_records.jsonl").write_text(json.dumps(decision_record) + "\n", encoding="utf-8")

(state_dir / "attack_response.json").write_text(
    json.dumps(
        {
            "target_repo": repo,
            "target_issue": int(issue),
            "baseline_id": baseline_id,
            "decision": decision_record["decision"],
            "enforcement": decision_record["enforcement"],
            "request_comment_body": comment_body,
        },
        indent=2,
    ),
    encoding="utf-8",
)
PY

echo "trial executed (live): $ATTACK_PROFILE_ID"
