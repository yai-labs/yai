#!/usr/bin/env bash
set -euo pipefail

: "${WS:=dev}"

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/../../../.." && pwd)"
DATASET_DIR="$ROOT_DIR/datasets/global-stress/v1"
NODES="$DATASET_DIR/seed/semantic_nodes.jsonl"
EDGES="$DATASET_DIR/seed/semantic_edges.jsonl"
BIN="${BIN:-$(command -v yai || true)}"

if [[ -z "$BIN" || ! -x "$BIN" ]]; then
  if [[ -x "$ROOT_DIR/mind/target/release/yai" ]]; then
    BIN="$ROOT_DIR/mind/target/release/yai"
  fi
fi

if [[ -z "$BIN" || ! -x "$BIN" ]]; then
  echo "FAIL: yai binary not found (set BIN or build mind/target/release/yai)"
  exit 1
fi

echo "Importing seed nodes into ws=$WS"
while IFS= read -r line; do
  [[ -z "${line// }" ]] && continue
  [[ "${line#"${line%%[![:space:]]*}"}" == \#* ]] && continue
  parsed=$(python3 - <<'PY'
import json,sys
raw=sys.stdin.read().strip()
if not raw:
    sys.exit(0)
try:
    o=json.loads(raw)
except Exception:
    sys.exit(0)
node_id=o.get("id")
kind=o.get("kind")
meta=o.get("meta",{})
if not isinstance(node_id,str) or not node_id or not node_id.startswith("node:"):
    sys.exit(0)
if not isinstance(kind,str) or not kind:
    sys.exit(0)
print(f"{node_id}\t{kind}\t{json.dumps(meta)}")
PY
<<<"$line")
  [[ -z "$parsed" ]] && continue
  IFS=$'\t' read -r id kind meta <<<"$parsed"
  [[ -z "$id" || -z "$kind" ]] && continue
  "$BIN" graph add-node --ws "$WS" --id "$id" --kind "$kind" --meta "$meta" >/dev/null
done < "$NODES"

echo "Importing seed edges into ws=$WS"
while IFS= read -r line; do
  [[ -z "${line// }" ]] && continue
  [[ "${line#"${line%%[![:space:]]*}"}" == \#* ]] && continue
  parsed=$(python3 - <<'PY'
import json,sys
raw=sys.stdin.read().strip()
if not raw:
    sys.exit(0)
try:
    o=json.loads(raw)
except Exception:
    sys.exit(0)
src=o.get("src")
dst=o.get("dst")
rel=o.get("rel")
weight=o.get("weight",1.0)
if not isinstance(src,str) or not src or not src.startswith("node:"):
    sys.exit(0)
if not isinstance(dst,str) or not dst or not dst.startswith("node:"):
    sys.exit(0)
if not isinstance(rel,str) or not rel:
    sys.exit(0)
try:
    w=float(weight)
except Exception:
    w=1.0
print(f"{src}\t{dst}\t{rel}\t{w}")
PY
<<<"$line")
  [[ -z "$parsed" ]] && continue
  IFS=$'\t' read -r src dst rel w <<<"$parsed"
  [[ -z "$src" || -z "$dst" || -z "$rel" ]] && continue
  "$BIN" graph add-edge --ws "$WS" --src "$src" --dst "$dst" --rel "$rel" --weight "$w" >/dev/null
done < "$EDGES"

echo "Done."
