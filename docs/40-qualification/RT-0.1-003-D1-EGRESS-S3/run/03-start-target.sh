#!/usr/bin/env bash
set -euo pipefail
source "$(dirname "$0")/_lib.sh"

curl --version >/dev/null 2>&1 || { echo "curl is required" >&2; exit 1; }

python3 - <<'PY'
import json, os, subprocess
out = subprocess.check_output(["curl", "--version"], text=True).splitlines()[0]
meta = {
  "tool": "curl",
  "version_line": out,
  "target_host": "127.0.0.1",
  "target_port": 8443,
  "status": "ready",
}
open(os.path.join(os.environ["STATE_DIR"], "target.json"), "w", encoding="utf-8").write(json.dumps(meta, indent=2))
PY

echo "target ready: curl + local endpoint plan"
