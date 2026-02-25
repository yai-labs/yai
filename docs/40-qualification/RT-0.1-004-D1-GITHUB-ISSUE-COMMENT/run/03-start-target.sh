#!/usr/bin/env bash
set -euo pipefail
source "$(dirname "$0")/_lib.sh"

if [[ -z "$GH_ISSUE_NUMBER" ]]; then
  echo "GH_ISSUE_NUMBER is required for RT-004" >&2
  exit 1
fi

gh auth status >/dev/null
# Real reachability precheck to the exact target effect path.
gh issue view "$GH_ISSUE_NUMBER" -R "$GH_REPO" --json number,title >/dev/null

python3 - <<'PY'
import datetime
import json
import os
from pathlib import Path

state_dir = Path(os.environ["STATE_DIR"])
state_dir.mkdir(parents=True, exist_ok=True)
(state_dir / "target.json").write_text(
    json.dumps(
        {
            "target_profile": os.environ.get("TARGET_PROFILE", "github"),
            "target_repo": os.environ["GH_REPO"],
            "target_issue": int(os.environ["GH_ISSUE_NUMBER"]),
            "target_url": os.environ["TARGET_URL"],
            "checked_at": datetime.datetime.now(datetime.UTC).isoformat(),
        },
        indent=2,
    ),
    encoding="utf-8",
)
PY

echo "target ready: profile=github repo=$GH_REPO issue=$GH_ISSUE_NUMBER"
