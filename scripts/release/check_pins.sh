#!/usr/bin/env bash
set -euo pipefail

YAI_SPECS_REPO="${YAI_SPECS_REPO:-https://github.com/francescomaiomascio/yai-specs.git}"
YAI_CLI_REPO="${YAI_CLI_REPO:-https://github.com/francescomaiomascio/yai-cli.git}"
STRICT_SPECS_HEAD="${STRICT_SPECS_HEAD:-1}"

ROOT="$(git rev-parse --show-toplevel 2>/dev/null || pwd)"
TMP_DIR="$(mktemp -d)"
cleanup() {
  rm -rf "$TMP_DIR"
}
trap cleanup EXIT

fail() {
  local code="$1"
  local msg="$2"
  echo "result=FAIL"
  echo "reason=$msg"
  echo "ERROR: $msg" >&2
  exit "$code"
}

if [ ! -d "$ROOT/deps/yai-specs/.git" ] && [ ! -f "$ROOT/deps/yai-specs/.git" ]; then
  fail 3 "deps/yai-specs is not a git repo; cannot verify pin"
fi

YAI_SPECS_PIN="$(git -C "$ROOT/deps/yai-specs" rev-parse HEAD 2>/dev/null || true)"
if ! echo "$YAI_SPECS_PIN" | grep -Eq '^[0-9a-f]{40}$'; then
  fail 3 "invalid yai specs pin from deps/yai-specs"
fi

YAI_CLI_MAIN_SHA="$(git ls-remote "$YAI_CLI_REPO" refs/heads/main | awk '{print $1}' | head -n1 || true)"
if ! echo "$YAI_CLI_MAIN_SHA" | grep -Eq '^[0-9a-f]{40}$'; then
  fail 3 "cannot resolve yai-cli main HEAD from $YAI_CLI_REPO"
fi

CLI_TMP="$TMP_DIR/yai-cli"
git clone --depth 1 "$YAI_CLI_REPO" "$CLI_TMP" >/dev/null 2>&1
YAI_CLI_SPECS_PIN="$(git -C "$CLI_TMP" rev-parse HEAD:deps/yai-specs 2>/dev/null || true)"
if ! echo "$YAI_CLI_SPECS_PIN" | grep -Eq '^[0-9a-f]{40}$'; then
  fail 3 "could not resolve yai-cli specs pin from gitlink deps/yai-specs"
fi

SPECS_HEAD="$(git ls-remote "$YAI_SPECS_REPO" refs/heads/main | awk '{print $1}' | head -n1 || true)"
if ! echo "$SPECS_HEAD" | grep -Eq '^[0-9a-f]{40}$'; then
  fail 3 "cannot resolve yai-specs main HEAD from $YAI_SPECS_REPO"
fi

CHECK_TMP="$TMP_DIR/specs-check"
git init -q "$CHECK_TMP"
git -C "$CHECK_TMP" remote add origin "$YAI_SPECS_REPO"
if ! git -C "$CHECK_TMP" fetch --depth 1 origin "$YAI_SPECS_PIN" >/dev/null 2>&1; then
  fail 3 "yai specs pin $YAI_SPECS_PIN is not reachable in $YAI_SPECS_REPO"
fi
if ! git -C "$CHECK_TMP" cat-file -e "${YAI_SPECS_PIN}^{commit}" >/dev/null 2>&1; then
  fail 3 "yai specs pin $YAI_SPECS_PIN is not a valid commit in $YAI_SPECS_REPO"
fi

echo "yai_pin=$YAI_SPECS_PIN"
echo "yai_cli_pin=$YAI_CLI_SPECS_PIN"
echo "yai_cli_main_head=$YAI_CLI_MAIN_SHA"
echo "yai_specs_main_head=$SPECS_HEAD"
echo "strict_specs_head=$STRICT_SPECS_HEAD"

if [ "$YAI_SPECS_PIN" != "$YAI_CLI_SPECS_PIN" ]; then
  fail 2 "pin mismatch between yai and yai-cli"
fi

if [ "$STRICT_SPECS_HEAD" = "1" ] && [ "$YAI_SPECS_PIN" != "$SPECS_HEAD" ]; then
  fail 4 "strict mode enabled and pin is not yai-specs/main HEAD"
fi

echo "result=PASS"
echo "reason=aligned specs pins"
echo "PASS: yai and yai-cli specs pins are aligned and valid."
