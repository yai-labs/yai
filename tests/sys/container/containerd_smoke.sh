#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/../../.." && pwd)"
BIN="$ROOT_DIR/build/bin/yai-containerd"

if [[ ! -x "$BIN" ]]; then
  echo "missing binary: $BIN" >&2
  exit 1
fi

TMP_HOME="$(mktemp -d /tmp/yai-containerd-test.XXXXXX)"
trap 'rm -rf "$TMP_HOME"' EXIT
export HOME="$TMP_HOME"

CID="sys-smoke"

"$BIN" create "$CID" managed
"$BIN" initialize "$CID"
"$BIN" open "$CID"
"$BIN" attach "$CID" 1001

OUT_ACTIVE="$("$BIN" show "$CID")"
echo "$OUT_ACTIVE" | grep -q "lifecycle=active"
echo "$OUT_ACTIVE" | grep -q "session_bound=1"
echo "$OUT_ACTIVE" | grep -q "active_session=1001 mode=normal"
echo "$OUT_ACTIVE" | grep -q "policy_view=1 grants_view=1"
PROJECTED_ROOT="$(echo "$OUT_ACTIVE" | sed -n 's/^projected-root=\([^ ]*\) backing-store=.*/\1/p')"
BACKING_STORE="$(echo "$OUT_ACTIVE" | sed -n 's/^projected-root=[^ ]* backing-store=\(.*\)$/\1/p')"
[[ -n "$PROJECTED_ROOT" ]]
[[ -n "$BACKING_STORE" ]]
[[ -d "$PROJECTED_ROOT" ]]
[[ -d "$BACKING_STORE" ]]
[[ -d "$PROJECTED_ROOT/system" ]]
[[ -d "$PROJECTED_ROOT/state" ]]
[[ -d "$PROJECTED_ROOT/data" ]]
[[ -d "$PROJECTED_ROOT/mounts" ]]
[[ -d "$PROJECTED_ROOT/runtime" ]]
[[ -d "$PROJECTED_ROOT/sessions" ]]
[[ -d "$PROJECTED_ROOT/logs" ]]
[[ -d "$PROJECTED_ROOT/tmp" ]]

RESOLVED_DATA="$("$BIN" resolve "$CID" /data/events.log)"
[[ "$RESOLVED_DATA" == "$PROJECTED_ROOT/data/events.log" ]]
if "$BIN" resolve "$CID" ../../etc/passwd >/dev/null 2>&1; then
  echo "expected traversal escape to fail" >&2
  exit 1
fi

"$BIN" mount "$CID" /host/secure-dump /mounts/diag rw hidden
if "$BIN" visible "$CID" /mounts/diag/trace.json >/dev/null 2>&1; then
  echo "expected hidden mount to be invisible" >&2
  exit 1
fi

"$BIN" mount "$CID" /host/ops /mounts/ops ro privileged-only
if "$BIN" visible "$CID" /mounts/ops/report.txt >/dev/null 2>&1; then
  echo "expected privileged-only mount to be hidden for non-privileged caller" >&2
  exit 1
fi
"$BIN" visible "$CID" /mounts/ops/report.txt 1 >/dev/null

"$BIN" enter "$CID" 1001 | grep -q "mode=normal"
if "$BIN" escape "$CID" 1001 admin >/dev/null 2>&1; then
  echo "expected admin escape to fail for normal session" >&2
  exit 1
fi

"$BIN" bind "$CID" 2001 privileged 255
OUT_PRIV="$("$BIN" show "$CID")"
echo "$OUT_PRIV" | grep -q "active_session=2001 mode=privileged"
"$BIN" enter "$CID" 2001 | grep -q "mode=privileged"
"$BIN" escape "$CID" 2001 admin >/dev/null

"$BIN" rebind "$CID" 2001 3001 recovery 127
OUT_REBOUND="$("$BIN" show "$CID")"
echo "$OUT_REBOUND" | grep -q "active_session=3001 mode=recovery"
"$BIN" recovery-enter "$CID" 3001 >/dev/null
"$BIN" leave "$CID" 3001
OUT_LEFT="$("$BIN" show "$CID")"
echo "$OUT_LEFT" | grep -q "session_bound=0"
echo "$OUT_LEFT" | grep -q "lifecycle=recovery"
echo "$OUT_LEFT" | grep -q "recovery-flags=1"

"$BIN" seal "$CID"
"$BIN" destroy "$CID"
OUT_DESTROYED="$("$BIN" show "$CID")"
echo "$OUT_DESTROYED" | grep -q "lifecycle=destroyed"

echo "containerd smoke: ok"
