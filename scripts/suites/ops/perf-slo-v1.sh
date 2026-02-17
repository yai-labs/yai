#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/../../.." && pwd)"
WS="${1:-perf_v1}"
ITERATIONS="${ITERATIONS:-40}"
P95_BUDGET_MS="${P95_BUDGET_MS:-2000}"
source "$ROOT/scripts/dev/resolve-yai-bin.sh"
BIN="$(yai_resolve_bin "$ROOT" || true)"

if [[ -z "$BIN" || ! -x "$BIN" ]]; then
  echo "FAIL: yai binary not found"
  exit 1
fi

cleanup() {
  "$BIN" down --ws "$WS" --force >/dev/null 2>&1 || true
}
trap cleanup EXIT

echo "== perf-slo-v1 (ws=$WS, iterations=$ITERATIONS, p95<=${P95_BUDGET_MS}ms)"

"$BIN" down --ws "$WS" --force >/dev/null 2>&1 || true
"$BIN" up --ws "$WS" --build --detach >/dev/null

STATUS_OUT="$("$BIN" status --ws "$WS")"
echo "$STATUS_OUT" | rg -q "runtime_sock_exists=true" || {
  echo "FAIL: runtime sock missing after up"
  exit 1
}

TMP_LAT="$(mktemp)"
python3 - "$BIN" "$WS" "$ITERATIONS" "$TMP_LAT" <<'PY'
import subprocess, sys, time
binp, ws, iters, out = sys.argv[1], sys.argv[2], int(sys.argv[3]), sys.argv[4]
vals = []
for _ in range(iters):
    t0 = time.perf_counter()
    proc = subprocess.run(
        [binp, "graph", "query", "--ws", ws, "--text", "runtime sock", "--k", "4"],
        stdout=subprocess.PIPE,
        stderr=subprocess.PIPE,
        text=True,
    )
    dt = (time.perf_counter() - t0) * 1000.0
    if proc.returncode != 0:
        print(proc.stderr.strip() or "graph query failed", file=sys.stderr)
        sys.exit(1)
    vals.append(dt)
with open(out, "w", encoding="utf-8") as f:
    for v in vals:
        f.write(f"{v:.3f}\n")
PY

STATS="$(python3 - "$TMP_LAT" <<'PY'
import sys
p = sys.argv[1]
vals = []
with open(p, "r", encoding="utf-8") as f:
    for line in f:
        line=line.strip()
        if line:
            vals.append(float(line))
vals.sort()
n=len(vals)
def pct(x):
    if n == 0:
        return 0.0
    idx=max(0, min(n-1, int(round((x/100.0)*(n-1)))))
    return vals[idx]
mn=vals[0] if vals else 0.0
mx=vals[-1] if vals else 0.0
avg=(sum(vals)/n) if n else 0.0
p50=pct(50)
p95=pct(95)
print(f"n={n} min_ms={mn:.2f} avg_ms={avg:.2f} p50_ms={p50:.2f} p95_ms={p95:.2f} max_ms={mx:.2f}")
print(f"{p95:.2f}")
PY
)"

echo "$STATS" | head -n1
P95="$(echo "$STATS" | tail -n1)"

RUN_DIR="$HOME/.yai/run/$WS"
SESSION_JSON="$RUN_DIR/session.json"
if [[ -f "$SESSION_JSON" ]]; then
  python3 - "$SESSION_JSON" <<'PY'
import json, os, subprocess, sys
p = sys.argv[1]
obj = json.load(open(p, "r", encoding="utf-8"))
for k in ("kernel_pid", "engine_pid", "mind_pid"):
    pid = obj.get(k)
    if not pid:
        continue
    cmd = ["ps", "-o", "rss=", "-p", str(pid)]
    out = subprocess.run(cmd, stdout=subprocess.PIPE, stderr=subprocess.DEVNULL, text=True)
    rss_kb = out.stdout.strip() or "n/a"
    print(f"{k}={pid} rss_kb={rss_kb}")
PY
fi

awk -v p95="$P95" -v b="$P95_BUDGET_MS" 'BEGIN { exit !(p95 <= b) }' || {
  echo "FAIL: p95 latency budget exceeded (${P95}ms > ${P95_BUDGET_MS}ms)"
  exit 1
}

rm -f "$TMP_LAT"
echo "OK: perf-slo-v1 passed"
