#!/usr/bin/env bash
set -euo pipefail

ROOT="$(git rev-parse --show-toplevel 2>/dev/null || pwd)"
MANIFEST_PATH="docs/proof/PP-FOUNDATION-0001/pp-foundation-0001.manifest.v1.json"
README_PATH="docs/proof/PP-FOUNDATION-0001/README.md"
TARGET_REF=""
DO_FETCH=1
DRY_RUN=0
DO_STAGE=1
DO_CHECK=1

usage() {
  cat <<'EOF'
Usage:
  tools/release/sync_specs_refs.sh --target <sha|ref> [options]

Options:
  --target <ref>   Required. Target commit/ref for deps/yai-specs (e.g. <sha>, origin/main).
  --manifest <p>   Manifest JSON path (default: docs/proof/PP-FOUNDATION-0001/pp-foundation-0001.manifest.v1.json)
  --readme <p>     Proof pack README path (default: docs/proof/PP-FOUNDATION-0001/README.md)
  --no-fetch       Do not run git fetch origin in deps/yai-specs
  --dry-run        Print planned changes only
  --no-stage       Do not git add changed files
  --no-check       Do not run tools/bin/yai-proof-check after update
  -h, --help       Show this help

Examples:
  tools/release/sync_specs_refs.sh --target b96573d751cf12ff756c9643f4acf926743a1226
  tools/release/sync_specs_refs.sh --target origin/main
EOF
}

while [[ $# -gt 0 ]]; do
  case "$1" in
    --target)
      TARGET_REF="${2:-}"
      shift 2
      ;;
    --manifest)
      MANIFEST_PATH="${2:-}"
      shift 2
      ;;
    --readme)
      README_PATH="${2:-}"
      shift 2
      ;;
    --no-fetch)
      DO_FETCH=0
      shift
      ;;
    --dry-run)
      DRY_RUN=1
      shift
      ;;
    --no-stage)
      DO_STAGE=0
      shift
      ;;
    --no-check)
      DO_CHECK=0
      shift
      ;;
    -h|--help)
      usage
      exit 0
      ;;
    *)
      echo "[sync-specs-refs] ERROR: unknown option: $1" >&2
      usage
      exit 2
      ;;
  esac
done

if [[ -z "$TARGET_REF" ]]; then
  echo "[sync-specs-refs] ERROR: --target is required" >&2
  usage
  exit 2
fi

SPECS_DIR="$ROOT/deps/yai-specs"
MANIFEST="$ROOT/$MANIFEST_PATH"
README="$ROOT/$README_PATH"

[[ -d "$SPECS_DIR/.git" || -f "$SPECS_DIR/.git" ]] || { echo "[sync-specs-refs] ERROR: missing git repo at deps/yai-specs" >&2; exit 2; }
[[ -f "$MANIFEST" ]] || { echo "[sync-specs-refs] ERROR: missing manifest: $MANIFEST_PATH" >&2; exit 2; }
[[ -f "$README" ]] || { echo "[sync-specs-refs] ERROR: missing readme: $README_PATH" >&2; exit 2; }

if [[ -n "$(git -C "$SPECS_DIR" status --porcelain)" ]]; then
  echo "[sync-specs-refs] ERROR: deps/yai-specs has uncommitted changes; refuse to switch pin" >&2
  exit 2
fi

if [[ $DO_FETCH -eq 1 ]]; then
  git -C "$SPECS_DIR" fetch origin
fi

if ! TARGET_SHA="$(git -C "$SPECS_DIR" rev-parse "$TARGET_REF" 2>/dev/null)"; then
  echo "[sync-specs-refs] ERROR: cannot resolve target ref in deps/yai-specs: $TARGET_REF" >&2
  exit 2
fi

if ! echo "$TARGET_SHA" | grep -Eq '^[0-9a-f]{40}$'; then
  echo "[sync-specs-refs] ERROR: resolved target is not a 40-char SHA: $TARGET_SHA" >&2
  exit 2
fi

CURRENT_SHA="$(git -C "$SPECS_DIR" rev-parse HEAD)"

echo "[sync-specs-refs] current specs pin : $CURRENT_SHA"
echo "[sync-specs-refs] target specs pin  : $TARGET_SHA"

if [[ "$CURRENT_SHA" == "$TARGET_SHA" ]]; then
  echo "[sync-specs-refs] specs pin already aligned"
else
  if [[ $DRY_RUN -eq 1 ]]; then
    echo "[sync-specs-refs] DRY-RUN: would checkout deps/yai-specs to $TARGET_SHA"
  else
    git -C "$SPECS_DIR" checkout "$TARGET_SHA"
  fi
fi

if [[ $DRY_RUN -eq 1 ]]; then
  echo "[sync-specs-refs] DRY-RUN: would update $MANIFEST_PATH pins.yai_specs.commit=$TARGET_SHA"
  echo "[sync-specs-refs] DRY-RUN: would update specs commit line in $README_PATH"
else
  python3 - "$MANIFEST" "$README" "$TARGET_SHA" <<'PY'
import json
import re
import sys
from pathlib import Path

manifest = Path(sys.argv[1])
readme = Path(sys.argv[2])
target = sys.argv[3]

# Update manifest pin
obj = json.loads(manifest.read_text(encoding="utf-8"))
obj.setdefault("pins", {}).setdefault("yai_specs", {})["commit"] = target
manifest.write_text(json.dumps(obj, indent=2) + "\n", encoding="utf-8")

# Update human README line (only the yai-specs commit token)
text = readme.read_text(encoding="utf-8")
pat = re.compile(r"(- `yai-specs`:.*?commit `)([0-9a-f]{40})(`.*)")
new_text, n = pat.subn(r"\g<1>" + target + r"\g<3>", text, count=1)
if n == 0:
    raise SystemExit("[sync-specs-refs] ERROR: could not find yai-specs commit line in proof README")
readme.write_text(new_text, encoding="utf-8")
PY
fi

if [[ $DO_STAGE -eq 1 && $DRY_RUN -eq 0 ]]; then
  git add deps/yai-specs "$MANIFEST_PATH" "$README_PATH"
  echo "[sync-specs-refs] staged: deps/yai-specs, $MANIFEST_PATH, $README_PATH"
fi

if [[ $DO_CHECK -eq 1 && $DRY_RUN -eq 0 ]]; then
  "$ROOT/tools/bin/yai-proof-check"
fi

echo "[sync-specs-refs] DONE"
