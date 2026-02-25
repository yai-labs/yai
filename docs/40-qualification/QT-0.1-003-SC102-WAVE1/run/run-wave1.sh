#!/usr/bin/env bash
set -euo pipefail

DIR="$(cd "$(dirname "$0")" && pwd)"
QUAL_ROOT="$(cd "$DIR/../.." && pwd)"

echo "[WAVE1] Stage 1/2: D1 live containment"
(
  cd "$QUAL_ROOT/QT-0.1-001-SC102"
  QT_MODE=live DOMAIN_PACK_ID=D1-digital/egress-v1 BASELINE_ID=baseline-deny ./run/run-three.sh
)

echo "[WAVE1] Stage 2/2: D8 docker params-lock deny"
(
  cd "$QUAL_ROOT/RT-0.1-001-D8-PARAMS-LOCK"
  BASELINE_ID=baseline-deny TARGET_PROFILE=docker ./run/run-three.sh
)

echo "[WAVE1] PASS: SC-102 Wave 1 (D1 + D8)"
