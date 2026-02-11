#!/usr/bin/env bash
set -euo pipefail

: "${WS:=dev}"

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
EVENTS="$ROOT_DIR/seed/episodic_events.jsonl"
RUN_DIR="${HOME}/.yai/run/${WS}"

mkdir -p "$RUN_DIR"
cp "$EVENTS" "${RUN_DIR}/events.log"

echo "OK: events.log loaded to ${RUN_DIR}/events.log"
