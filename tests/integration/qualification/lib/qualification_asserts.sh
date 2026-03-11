#!/usr/bin/env bash
set -euo pipefail

yai_qual_fail() {
  echo "$1"
  exit 1
}

yai_qual_assert_file() {
  local path="$1"
  local msg="$2"
  [[ -f "$path" ]] || yai_qual_fail "$msg (missing file: $path)"
}

yai_qual_assert_dir() {
  local path="$1"
  local msg="$2"
  [[ -d "$path" ]] || yai_qual_fail "$msg (missing dir: $path)"
}

yai_qual_assert_eq() {
  local got="$1"
  local want="$2"
  local msg="$3"
  [[ "$got" == "$want" ]] || yai_qual_fail "$msg (got=$got want=$want)"
}

yai_qual_assert_ge() {
  local got="$1"
  local min="$2"
  local msg="$3"
  (( got >= min )) || yai_qual_fail "$msg (got=$got min=$min)"
}
