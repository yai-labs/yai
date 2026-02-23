#!/usr/bin/env bash
set -euo pipefail

YAI_SPECS_REPO="${YAI_SPECS_REPO:-https://github.com/yai-labs/yai-specs.git}"
YAI_CLI_REPO="${YAI_CLI_REPO:-https://github.com/yai-labs/yai-cli.git}"
STRICT_SPECS_HEAD="${STRICT_SPECS_HEAD:-1}"

ROOT="$(git rev-parse --show-toplevel 2>/dev/null || pwd)"
TMP_DIR="$(mktemp -d)"
cleanup() {
  rm -rf "$TMP_DIR"
}
trap cleanup EXIT

short_sha() {
  local sha="${1:-}"
  if echo "$sha" | grep -Eq '^[0-9a-f]{40}$'; then
    printf "%s" "${sha:0:12}"
  else
    printf "%s" "$sha"
  fi
}

read_cli_sha_ref() {
  local ref_file="$ROOT/deps/yai-cli.ref"
  if [ ! -f "$ref_file" ]; then
    fail 3 "missing/invalid deps/yai-cli.ref"
  fi
  local cli_sha
  cli_sha="$(sed -n 's/^cli_sha=//p' "$ref_file" | tr -d '\r' | head -n1)"
  if ! echo "$cli_sha" | grep -Eq '^[0-9a-f]{40}$'; then
    fail 3 "missing/invalid deps/yai-cli.ref"
  fi
  printf "%s" "$cli_sha"
}

extract_specs_gitlink() {
  local repo_dir="$1"
  local ref="${2:-HEAD}"
  git -C "$repo_dir" ls-tree -d "$ref" deps/yai-specs | awk '{print $3}' | head -n1
}

print_fix_plan() {
  local expected_sha="$1"
  local yai_pin="$2"
  local yai_cli_pin="$3"
  local short="${expected_sha:0:7}"

  cat <<EOF
[SUMMARY]
  expected_specs_sha : ${expected_sha}
  yai_pin            : ${yai_pin}
  yai_cli_pin        : ${yai_cli_pin}

Fix (required before release):

  export YAI_WORKSPACE="<path-to-your-yai-workspace>"

  yai-cli (bump deps/yai-specs to ${expected_sha})
    cd "\$YAI_WORKSPACE/yai-cli"
    git checkout main && git pull --rebase
    git checkout -b chore/bump-specs-${short}
    git -C deps/yai-specs fetch origin
    git -C deps/yai-specs checkout ${expected_sha}
    git add deps/yai-specs
    git commit -m "chore(specs): bump yai-specs pin to ${short} in yai-cli"
    git push -u origin chore/bump-specs-${short}

  yai (bump deps/yai-specs to ${expected_sha})
    cd "\$YAI_WORKSPACE/yai"
    git checkout main && git pull --rebase
    git checkout -b chore/bump-specs-${short}
    git -C deps/yai-specs fetch origin
    git -C deps/yai-specs checkout ${expected_sha}
    git add deps/yai-specs
    git commit -m "chore(specs): bump yai-specs pin to ${short} in yai"
    git push -u origin chore/bump-specs-${short}

  close bump branches (after PR merge to main)
    cd "\$YAI_WORKSPACE/yai-cli"
    git checkout main && git pull --rebase
    git branch -d chore/bump-specs-${short} || git branch -D chore/bump-specs-${short}
    git push origin --delete chore/bump-specs-${short} || true

    cd "\$YAI_WORKSPACE/yai"
    git checkout main && git pull --rebase
    git branch -d chore/bump-specs-${short} || git branch -D chore/bump-specs-${short}
    git push origin --delete chore/bump-specs-${short} || true
EOF
}

print_triangle_fix_plan() {
  local expected_sha="$1"
  local cli_sha="$2"
  local cli_ref_specs_pin="$3"
  local shortspecs="${expected_sha:0:7}"

  cat <<EOF
[SUMMARY]
  expected_specs_sha : ${expected_sha}
  yai_cli_ref_sha    : ${cli_sha}
  yai_cli_ref_specs  : ${cli_ref_specs_pin}

Fix (required before release):

  export YAI_WORKSPACE="<path-to-your-yai-workspace>"

  yai-cli (choose/prepare an aligned commit pinned to ${expected_sha})
    cd "\$YAI_WORKSPACE/yai-cli"
    git checkout main && git pull --rebase
    git rev-parse HEAD
    git ls-tree -d HEAD deps/yai-specs
    # if deps/yai-specs gitlink is not ${expected_sha}, bump specs pin in yai-cli first.
    # when ready, keep HEAD as aligned commit:
    NEW_CLI_SHA=\$(git rev-parse HEAD)

  yai (update deps/yai-cli.ref to aligned CLI commit)
    cd "\$YAI_WORKSPACE/yai"
    git checkout main && git pull --rebase
    git checkout -b chore/bump-cli-ref-${shortspecs}
    printf "cli_sha=%s\n" "\$NEW_CLI_SHA" > deps/yai-cli.ref
    git add deps/yai-cli.ref
    git commit -m "chore(release): pin yai-cli to \${NEW_CLI_SHA:0:7}"
    git push -u origin chore/bump-cli-ref-${shortspecs}
EOF
}

fail() {
  local code="$1"
  local msg="$2"
  local expected_sha="${3:-}"
  local yai_pin="${4:-unknown}"
  local yai_cli_pin="${5:-unknown}"
  local yai_cli_ref_sha="${6:-unknown}"
  local yai_cli_ref_specs_pin="${7:-unknown}"
  echo
  echo "[RESULT] FAIL"
  echo "[REASON] $msg"
  if [ -n "$expected_sha" ] && { [ "$code" -eq 2 ] || [ "$code" -eq 3 ] || [ "$code" -eq 4 ]; }; then
    print_fix_plan "$expected_sha" "$yai_pin" "$yai_cli_pin"
  fi
  if [ -n "$expected_sha" ] && [ "$code" -eq 5 ]; then
    print_triangle_fix_plan "$expected_sha" "$yai_cli_ref_sha" "$yai_cli_ref_specs_pin"
  fi
  echo
  echo "[MACHINE]"
  echo "result=FAIL"
  echo "reason=$msg"
  echo "exit_code=$code"
  echo "yai_pin=$yai_pin"
  echo "yai_cli_pin=$yai_cli_pin"
  echo "yai_cli_ref_sha=$yai_cli_ref_sha"
  echo "yai_cli_ref_specs_pin=$yai_cli_ref_specs_pin"
  if [ -n "$expected_sha" ]; then
    echo "expected_specs_sha=$expected_sha"
  fi
  exit "$code"
}

if [ ! -d "$ROOT/deps/yai-specs/.git" ] && [ ! -f "$ROOT/deps/yai-specs/.git" ]; then
  fail 3 "deps/yai-specs is not a git repo; cannot verify pin"
fi

YAI_SPECS_PIN="$(git -C "$ROOT/deps/yai-specs" rev-parse HEAD 2>/dev/null || true)"
if ! echo "$YAI_SPECS_PIN" | grep -Eq '^[0-9a-f]{40}$'; then
  fail 3 "invalid yai specs pin from deps/yai-specs"
fi

CLI_SHA="$(read_cli_sha_ref)"

YAI_CLI_MAIN_SHA="$(git ls-remote "$YAI_CLI_REPO" refs/heads/main | awk '{print $1}' | head -n1 || true)"
if ! echo "$YAI_CLI_MAIN_SHA" | grep -Eq '^[0-9a-f]{40}$'; then
  fail 3 "cannot resolve yai-cli main HEAD from $YAI_CLI_REPO"
fi

CLI_MAIN_TMP="$TMP_DIR/yai-cli-main"
git clone --no-checkout "$YAI_CLI_REPO" "$CLI_MAIN_TMP" >/dev/null 2>&1
if ! git -C "$CLI_MAIN_TMP" fetch --depth 1 origin "$YAI_CLI_MAIN_SHA" >/dev/null 2>&1; then
  fail 3 "cannot fetch yai-cli main commit $YAI_CLI_MAIN_SHA from $YAI_CLI_REPO"
fi
if ! git -C "$CLI_MAIN_TMP" checkout -q "$YAI_CLI_MAIN_SHA" >/dev/null 2>&1; then
  fail 3 "cannot checkout yai-cli main commit $YAI_CLI_MAIN_SHA"
fi
YAI_CLI_SPECS_PIN="$(extract_specs_gitlink "$CLI_MAIN_TMP" HEAD)"
if ! echo "$YAI_CLI_SPECS_PIN" | grep -Eq '^[0-9a-f]{40}$'; then
  fail 3 "could not resolve yai-cli specs pin from gitlink deps/yai-specs"
fi

CLI_REF_TMP="$TMP_DIR/yai-cli-ref"
git clone --no-checkout "$YAI_CLI_REPO" "$CLI_REF_TMP" >/dev/null 2>&1
if ! git -C "$CLI_REF_TMP" fetch --depth 1 origin "$CLI_SHA" >/dev/null 2>&1; then
  fail 3 "cannot fetch yai-cli ref commit $CLI_SHA from $YAI_CLI_REPO"
fi
if ! git -C "$CLI_REF_TMP" checkout -q "$CLI_SHA" >/dev/null 2>&1; then
  fail 3 "cannot checkout yai-cli ref commit $CLI_SHA"
fi
CLI_REF_SPECS_PIN="$(extract_specs_gitlink "$CLI_REF_TMP" HEAD)"
if ! echo "$CLI_REF_SPECS_PIN" | grep -Eq '^[0-9a-f]{40}$'; then
  fail 3 "could not resolve specs pin for yai-cli.ref commit $CLI_SHA"
fi

SPECS_HEAD="$(git ls-remote "$YAI_SPECS_REPO" refs/heads/main | awk '{print $1}' | head -n1 || true)"
if ! echo "$SPECS_HEAD" | grep -Eq '^[0-9a-f]{40}$'; then
  fail 3 "cannot resolve yai-specs main HEAD from $YAI_SPECS_REPO"
fi

CHECK_TMP="$TMP_DIR/specs-check"
git init -q "$CHECK_TMP"
git -C "$CHECK_TMP" remote add origin "$YAI_SPECS_REPO"
if ! git -C "$CHECK_TMP" fetch --depth 1 origin "$YAI_SPECS_PIN" >/dev/null 2>&1; then
  fail 3 "yai specs pin $YAI_SPECS_PIN is not reachable in $YAI_SPECS_REPO" "$SPECS_HEAD" "$YAI_SPECS_PIN" "$YAI_CLI_SPECS_PIN"
fi
if ! git -C "$CHECK_TMP" cat-file -e "${YAI_SPECS_PIN}^{commit}" >/dev/null 2>&1; then
  fail 3 "yai specs pin $YAI_SPECS_PIN is not a valid commit in $YAI_SPECS_REPO" "$SPECS_HEAD" "$YAI_SPECS_PIN" "$YAI_CLI_SPECS_PIN"
fi

echo "[CHECK]"
echo "  yai_pin            : $(short_sha "$YAI_SPECS_PIN")"
echo "  yai_cli_pin        : $(short_sha "$YAI_CLI_SPECS_PIN")"
echo "  yai_cli_ref_sha    : $(short_sha "$CLI_SHA")"
echo "  yai_cli_ref_specs  : $(short_sha "$CLI_REF_SPECS_PIN")"
echo "  yai_cli_main_head  : $(short_sha "$YAI_CLI_MAIN_SHA")"
echo "  yai_specs_main_head: $(short_sha "$SPECS_HEAD")"
echo "  strict_specs_head  : $STRICT_SPECS_HEAD"

if [ "$YAI_SPECS_PIN" != "$YAI_CLI_SPECS_PIN" ]; then
  fail 2 "pin mismatch between yai and yai-cli" "$SPECS_HEAD" "$YAI_SPECS_PIN" "$YAI_CLI_SPECS_PIN"
fi

if [ "$STRICT_SPECS_HEAD" = "1" ] && [ "$YAI_SPECS_PIN" != "$SPECS_HEAD" ]; then
  fail 4 "strict mode enabled and pin is not yai-specs/main HEAD" "$SPECS_HEAD" "$YAI_SPECS_PIN" "$YAI_CLI_SPECS_PIN" "$CLI_SHA" "$CLI_REF_SPECS_PIN"
fi

EXPECTED_SPECS_SHA="$YAI_SPECS_PIN"
if [ "$STRICT_SPECS_HEAD" = "1" ]; then
  EXPECTED_SPECS_SHA="$SPECS_HEAD"
fi
echo "  expected_specs_sha : $(short_sha "$EXPECTED_SPECS_SHA")"

if [ "$CLI_REF_SPECS_PIN" != "$EXPECTED_SPECS_SHA" ]; then
  fail 5 "yai-cli.ref commit is not aligned to expected specs pin" "$EXPECTED_SPECS_SHA" "$YAI_SPECS_PIN" "$YAI_CLI_SPECS_PIN" "$CLI_SHA" "$CLI_REF_SPECS_PIN"
fi

REQUIRED_SPECS_PATHS=(
  "$ROOT/deps/yai-specs/VERSION"
  "$ROOT/deps/yai-specs/SPEC_MAP.md"
  "$ROOT/deps/yai-specs/specs/protocol/include/protocol.h"
  "$ROOT/deps/yai-specs/specs/vault/include/yai_vault_abi.h"
  "$ROOT/deps/yai-specs/formal/tla/YAI_KERNEL.tla"
  "$ROOT/deps/yai-specs/tools/release/bump_version.sh"
)
for path in "${REQUIRED_SPECS_PATHS[@]}"; do
  [ -f "$path" ] || fail 3 "missing required specs path: ${path#$ROOT/}"
done

if [ "${STRICT_BUNDLE_ENTRYPOINT:-0}" = "1" ]; then
  [ -x "$ROOT/tools/bundle/build_bundle.sh" ] || fail 3 "missing executable tools/bundle/build_bundle.sh"
  [ -x "$ROOT/tools/bundle/manifest.sh" ] || fail 3 "missing executable tools/bundle/manifest.sh"
  grep -qE '^bundle:' "$ROOT/Makefile" || fail 3 "Makefile missing bundle target"
  echo "[CHECK] bundle entrypoint scripts present"
fi

echo
echo "[RESULT] PASS"
echo "[REASON] aligned specs pins and yai-cli.ref triangle"
echo
echo "[MACHINE]"
echo "result=PASS"
echo "reason=aligned specs pins and yai-cli.ref triangle"
echo "exit_code=0"
echo "yai_pin=$YAI_SPECS_PIN"
echo "yai_cli_pin=$YAI_CLI_SPECS_PIN"
echo "yai_cli_ref_sha=$CLI_SHA"
echo "yai_cli_ref_specs_pin=$CLI_REF_SPECS_PIN"
echo "expected_specs_sha=$EXPECTED_SPECS_SHA"
echo "PASS: yai, yai-cli, and yai-cli.ref are aligned and valid."
