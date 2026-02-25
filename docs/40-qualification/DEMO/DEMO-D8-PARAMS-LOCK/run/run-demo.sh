#!/usr/bin/env bash
set -euo pipefail
DIR="$(cd "$(dirname "$0")" && pwd)"
# shellcheck source=/dev/null
source "$DIR/_lib.sh"

WS="${WS:-demo-d8}"
MANIFEST_JSON="$DEMO_DIR/MANIFEST.json"
YAI_SHA="$(git -C "$REPO_ROOT" rev-parse HEAD 2>/dev/null || echo unknown)"
YAI_CLI_SHA="$(git -C "$REPO_ROOT/../yai-cli" rev-parse HEAD 2>/dev/null || echo unknown)"
SPECS_SHA="$(git -C "$REPO_ROOT/deps/yai-law" rev-parse HEAD 2>/dev/null || echo unknown)"
VERSION_LABEL="dev-${YAI_SHA:0:7}"

cmd_capture "$CLI_DIR/01-yai-version.txt" yai --version
cmd_capture "$CLI_DIR/02-up.txt" yai up --ws "$WS" --detach --allow-degraded
cmd_capture "$CLI_DIR/03-root-ping.txt" yai root ping
cmd_capture "$CLI_DIR/04-kernel-ping.txt" yai kernel --arming --role operator ping

{
  echo "$ cd docs/40-qualification/RT-0.1-001-D8-PARAMS-LOCK"
  echo "$ BASELINE_ID=baseline-deny TARGET_PROFILE=docker ./run/run-three.sh"
} > "$CLI_DIR/05-manual-trial-command.txt"

tail -n 120 "$HOME/.yai/run/root/root.log" > "$LOG_DIR/root-tail.log" 2>&1 || true
tail -n 120 "$HOME/.yai/run/kernel/kernel.log" > "$LOG_DIR/kernel-tail.log" 2>&1 || true
tail -n 120 "$HOME/.yai/run/engine/engine.log" > "$LOG_DIR/engine-tail.log" 2>&1 || true

python3 - <<PY
import json, datetime
from pathlib import Path
p = Path(r"$MANIFEST_JSON")
man = json.loads(p.read_text())
man["generated_at"] = datetime.datetime.now(datetime.timezone.utc).isoformat()
man["product_release"] = {
  "yai_git_sha": "$YAI_SHA",
  "yai_cli_git_sha": "$YAI_CLI_SHA",
  "specs_pin_sha": "$SPECS_SHA",
  "version_label": "$VERSION_LABEL"
}
p.write_text(json.dumps(man, indent=2) + "\n")
PY

echo "[DEMO-D8] Runtime prepared and captures saved under $ARTIFACTS_DIR"
