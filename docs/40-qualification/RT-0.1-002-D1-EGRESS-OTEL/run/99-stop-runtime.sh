#!/usr/bin/env bash
set -euo pipefail
source "$(dirname "$0")/_lib.sh"

python3 - <<'PY2'
import json, os, signal
p=os.path.join(os.environ['STATE_DIR'],'pids.json')
try:
    data=json.load(open(p,'r',encoding='utf-8'))
except Exception:
    data={}
for key in ('engine_pid','root_pid','kernel_pid','boot_pid'):
    pid=int(data.get(key,0) or 0)
    if pid>0:
        try:
            os.kill(pid, signal.SIGTERM)
        except Exception:
            pass
PY2

sleep 0.4
rm -f "$ROOT_SOCK" "$KERNEL_SOCK" "$ENGINE_SOCK"

if [[ -n "$YAI_BIN" ]]; then
  "$YAI_BIN" kernel ws destroy "$WS_ID" --arming --role operator >/dev/null 2>&1 || true
fi
