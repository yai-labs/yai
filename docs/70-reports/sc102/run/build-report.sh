#!/usr/bin/env bash
set -euo pipefail

DIR="$(cd "$(dirname "$0")/.." && pwd)"
SRC="$DIR/SC102-REPORT-v0.1.0-draft.md"
OUT="$DIR/pdf/SC102-REPORT-v0.1.0-draft.pdf"

if ! command -v pandoc >/dev/null 2>&1; then
  echo "[report] WARN: pandoc not found; skipping PDF build (non-fatal)"
  echo "[report] To build manually: pandoc $SRC -o $OUT"
  exit 0
fi

if pandoc "$SRC" -o "$OUT"; then
  echo "[report] built: $OUT"
  exit 0
fi

echo "[report] WARN: pandoc present but PDF engine/deps missing; skipping (non-fatal)"
echo "[report] To build manually install LaTeX engine (e.g. pdflatex) and rerun"
exit 0
