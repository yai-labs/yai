#!/usr/bin/env bash
set -euo pipefail
DIR="$(cd "$(dirname "$0")" && pwd)"
# shellcheck source=/dev/null
source "$DIR/_lib.sh"

YAI_SHA="$(git -C "$REPO_ROOT" rev-parse HEAD 2>/dev/null || echo unknown)"
YAI_CLI_SHA="$(git -C "$REPO_ROOT/../yai-cli" rev-parse HEAD 2>/dev/null || echo unknown)"
SPECS_SHA="$(git -C "$REPO_ROOT/deps/yai-law" rev-parse HEAD 2>/dev/null || echo unknown)"
VERSION_LABEL="dev-${YAI_SHA:0:7}"

cmd_capture "$CLI_DIR/01-yai-version.txt" yai --version
cmd_capture "$CLI_DIR/02-root-ping.txt" yai root ping
cmd_capture "$CLI_DIR/03-kernel-ping.txt" yai kernel --arming --role operator ping

{
  echo "$ cd docs/40-qualification/QT-0.1-003-SC102-WAVE1"
  echo "$ ./run/run-wave.sh"
} > "$CLI_DIR/04-wave-command.txt"

python3 - <<PY
import json, platform, datetime
from pathlib import Path
p = Path(r"$MANIFEST")
man = json.loads(p.read_text())
man["generated_at"] = datetime.datetime.now(datetime.timezone.utc).isoformat()
man["product_release"]["yai_git_sha"] = "$YAI_SHA"
man["product_release"]["yai_cli_git_sha"] = "$YAI_CLI_SHA"
man["product_release"]["specs_pin_sha"] = "$SPECS_SHA"
man["product_release"]["version_label"] = "$VERSION_LABEL"
man["machine"] = {"system": platform.system(), "release": platform.release(), "machine": platform.machine()}
p.write_text(json.dumps(man, indent=2) + "\n")
PY

echo "[DEMO] Prepared. Execute RUNBOOK steps in $DEMO_DIR/RUNBOOK.md"
