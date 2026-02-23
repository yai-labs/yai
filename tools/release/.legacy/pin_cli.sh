#!/usr/bin/env bash
set -euo pipefail

ROOT="$(git rev-parse --show-toplevel 2>/dev/null || pwd)"
PIN_FILE="$ROOT/deps/yai-cli.ref"
YAI_CLI_REPO="${YAI_CLI_REPO:-https://github.com/yai-labs/yai-cli.git}"
YAI_CLI_BRANCH="${YAI_CLI_BRANCH:-main}"
EXPECTED_SPECS_SHA="${EXPECTED_SPECS_SHA:-}"

cleanup_tmp() {
  if [ -n "${TMP_DIR:-}" ] && [ -d "${TMP_DIR:-}" ]; then
    rm -rf "$TMP_DIR"
  fi
}
trap cleanup_tmp EXIT

TMP_DIR="$(mktemp -d)"

is_sha40() {
  echo "${1:-}" | tr -d '\r' | grep -Eq '^[0-9a-f]{40}$'
}

ensure_commit_exists() {
  local sha="$1"
  local repo_dir="$TMP_DIR/verify-sha"
  rm -rf "$repo_dir"
  git init -q "$repo_dir"
  git -C "$repo_dir" remote add origin "$YAI_CLI_REPO"
  if ! git -C "$repo_dir" fetch --depth 1 origin "$sha" >/dev/null 2>&1; then
    echo "ERROR: commit $sha is not fetchable from $YAI_CLI_REPO" >&2
    exit 1
  fi
  if ! git -C "$repo_dir" cat-file -e "${sha}^{commit}" >/dev/null 2>&1; then
    echo "ERROR: commit $sha is not a valid commit in $YAI_CLI_REPO" >&2
    exit 1
  fi
}

extract_specs_gitlink() {
  local repo_dir="$1"
  git -C "$repo_dir" ls-tree -d HEAD deps/yai-specs | awk '{print $3}' | head -n1
}

resolve_from_sha() {
  local sha="$1"
  sha="$(printf "%s" "$sha" | tr -d '\r')"
  if ! is_sha40 "$sha"; then
    echo "ERROR: YAI_CLI_SHA must be a 40-hex commit SHA" >&2
    exit 1
  fi
  ensure_commit_exists "$sha"
  printf "%s\n" "$sha"
}

resolve_from_tag() {
  local tag="$1"
  local sha=""
  sha="$(git ls-remote --tags "$YAI_CLI_REPO" "refs/tags/${tag}^{}" | awk '{print $1}' | head -n1 || true)"
  if [ -z "$sha" ]; then
    sha="$(git ls-remote --tags "$YAI_CLI_REPO" "refs/tags/${tag}" | awk '{print $1}' | head -n1 || true)"
  fi
  if [ -z "$sha" ]; then
    echo "ERROR: could not resolve YAI_CLI_TAG=$tag in $YAI_CLI_REPO" >&2
    exit 1
  fi
  ensure_commit_exists "$sha"
  printf "%s\n" "$sha"
}

resolve_from_branch() {
  local branch="$1"
  local sha=""
  sha="$(git ls-remote --heads "$YAI_CLI_REPO" "refs/heads/$branch" | awk '{print $1}' | head -n1 || true)"
  if [ -z "$sha" ]; then
    echo "ERROR: could not resolve YAI_CLI_BRANCH=$branch in $YAI_CLI_REPO" >&2
    exit 1
  fi
  ensure_commit_exists "$sha"
  printf "%s\n" "$sha"
}

verify_expected_specs_alignment() {
  local cli_sha="$1"
  local expected="$2"
  if ! is_sha40 "$expected"; then
    echo "ERROR: EXPECTED_SPECS_SHA must be a 40-hex commit SHA" >&2
    exit 1
  fi
  local repo_dir="$TMP_DIR/check-cli-specs"
  rm -rf "$repo_dir"
  git clone --no-checkout "$YAI_CLI_REPO" "$repo_dir" >/dev/null 2>&1
  if ! git -C "$repo_dir" fetch --depth 1 origin "$cli_sha" >/dev/null 2>&1; then
    echo "ERROR: cannot fetch resolved CLI sha $cli_sha from $YAI_CLI_REPO" >&2
    exit 1
  fi
  git -C "$repo_dir" checkout -q "$cli_sha"
  local cli_specs_sha
  cli_specs_sha="$(extract_specs_gitlink "$repo_dir")"
  if ! is_sha40 "$cli_specs_sha"; then
    echo "ERROR: could not resolve deps/yai-specs gitlink for resolved CLI sha $cli_sha" >&2
    exit 1
  fi
  if [ "$cli_specs_sha" != "$expected" ]; then
    echo "ERROR: resolved CLI sha is not aligned to EXPECTED_SPECS_SHA; update yai-cli or choose a different ref" >&2
    echo "resolved_cli_sha=$cli_sha" >&2
    echo "cli_specs_pin=$cli_specs_sha" >&2
    echo "expected_specs_sha=$expected" >&2
    exit 1
  fi
}

main() {
  local resolved=""
  if [ -n "${YAI_CLI_SHA:-}" ]; then
    resolved="$(resolve_from_sha "$YAI_CLI_SHA")"
  elif [ -n "${YAI_CLI_TAG:-}" ]; then
    resolved="$(resolve_from_tag "$YAI_CLI_TAG")"
  else
    resolved="$(resolve_from_branch "$YAI_CLI_BRANCH")"
  fi

  if ! is_sha40 "$resolved"; then
    echo "ERROR: resolved yai-cli reference is not a 40-hex SHA: $resolved" >&2
    exit 1
  fi

  if [ -n "$EXPECTED_SPECS_SHA" ]; then
    verify_expected_specs_alignment "$resolved" "$EXPECTED_SPECS_SHA"
  fi

  printf "cli_sha=%s\n" "$resolved" > "$PIN_FILE"
  printf "%s\n" "$resolved"
}

main "$@"
