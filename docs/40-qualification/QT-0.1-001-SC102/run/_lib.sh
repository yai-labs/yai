#!/usr/bin/env bash
set -euo pipefail

QT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
REPO_ROOT="$(cd "$QT_DIR/../../.." && pwd)"

DOMAIN_PACK_ID="${DOMAIN_PACK_ID:-D1-digital/egress-v1}"
BASELINE_ID="${BASELINE_ID:-baseline-deny}"
WORKLOAD_ID="${WORKLOAD_ID:-wrk-d1-egress-sim-v1}"
ATTACK_PROFILE_ID="${ATTACK_PROFILE_ID:-safe-egress-attempt-001}"
RUN_ID="${RUN_ID:-run-001}"

PACK_DIR="$REPO_ROOT/docs/30-catalog/domains/packs/$DOMAIN_PACK_ID"
BASELINE_FILE="$PACK_DIR/contracts/${BASELINE_ID}.json"
EXPECTED_FILE="$PACK_DIR/vectors/expected_outcomes.json"
EVIDENCE_DIR="$QT_DIR/evidence/$DOMAIN_PACK_ID/$RUN_ID"
STATE_DIR="$QT_DIR/run/.state/$RUN_ID"

mkdir -p "$EVIDENCE_DIR" "$STATE_DIR"

if [[ ! -f "$BASELINE_FILE" ]]; then
  echo "missing baseline file: $BASELINE_FILE" >&2
  exit 1
fi
if [[ ! -f "$EXPECTED_FILE" ]]; then
  echo "missing expected outcomes: $EXPECTED_FILE" >&2
  exit 1
fi

export QT_DIR REPO_ROOT DOMAIN_PACK_ID BASELINE_ID WORKLOAD_ID ATTACK_PROFILE_ID RUN_ID
export PACK_DIR BASELINE_FILE EXPECTED_FILE EVIDENCE_DIR STATE_DIR
