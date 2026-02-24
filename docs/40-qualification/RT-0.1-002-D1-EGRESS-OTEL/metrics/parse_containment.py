#!/usr/bin/env python3
import json
import os
import sys

if len(sys.argv) != 3:
    print("usage: parse_containment.py <decision_records.jsonl> <out.json>", file=sys.stderr)
    sys.exit(1)

src, dst = sys.argv[1], sys.argv[2]
if not os.path.exists(src):
    print(f"missing source: {src}", file=sys.stderr)
    sys.exit(1)

rows = []
with open(src, "r", encoding="utf-8") as f:
    for line in f:
        line = line.strip()
        if not line:
            continue
        rows.append(json.loads(line))
if not rows:
    print("no decision rows", file=sys.stderr)
    sys.exit(1)

r = rows[-1]
metrics = r.get("metrics", {})
out = {
    "runs": len(rows),
    "decision_outcome": r.get("decision", {}).get("outcome"),
    "reason_code": r.get("decision", {}).get("reason_code"),
    "time_to_contain_ms": int(metrics.get("time_to_contain_ms", 0)),
    "connect_established": bool(metrics.get("connect_established", False)),
    "bytes_exfiltrated": int(metrics.get("bytes_exfiltrated", 0)),
    "local_target_reachable": bool(metrics.get("local_target_reachable", False)),
    "precheck_hits": int(metrics.get("precheck_hits", 0)),
    "gated_hits": int(metrics.get("gated_hits", 0)),
    "forbidden_effect_success_rate": 1.0 if bool(metrics.get("connect_established", False)) else 0.0,
    "forbidden_effect_block_rate": 0.0 if bool(metrics.get("connect_established", False)) else 1.0,
    "containment_success_rate": 0.0 if bool(metrics.get("connect_established", False)) else 1.0,
}
with open(dst, "w", encoding="utf-8") as f:
    json.dump(out, f, indent=2)
