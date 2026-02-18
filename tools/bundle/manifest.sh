#!/usr/bin/env bash
set -euo pipefail

if [ "$#" -ne 9 ]; then
  echo "usage: $0 <stage_dir> <bundle_version> <core_version> <core_git_sha> <cli_ref> <cli_git_sha> <specs_git_sha> <os> <arch>" >&2
  exit 1
fi

STAGE_DIR="$1"
BUNDLE_VERSION="$2"
CORE_VERSION="$3"
CORE_GIT_SHA="$4"
CLI_REF="$5"
CLI_GIT_SHA="$6"
SPECS_GIT_SHA="$7"
PLATFORM_OS="$8"
PLATFORM_ARCH="$9"
CLI_REF_SHA="$CLI_REF"

BIN_DIR="$STAGE_DIR/bin"
OUT_MANIFEST="$STAGE_DIR/manifest.json"

fail() { echo "ERROR: $*" >&2; exit 1; }

for req in "$STAGE_DIR" "$BUNDLE_VERSION" "$CORE_VERSION" "$CORE_GIT_SHA" "$CLI_REF" "$CLI_GIT_SHA" "$SPECS_GIT_SHA" "$PLATFORM_OS" "$PLATFORM_ARCH"; do
  if [ -z "$req" ]; then
    fail "manifest requires non-empty fields"
  fi
done

# core git sha must be available
[ -n "${CORE_GIT_SHA:-}" ] || fail "core.git_sha missing"
echo "$CORE_GIT_SHA" | grep -Eq '^[0-9a-f]{40}$' || fail "core.git_sha invalid: $CORE_GIT_SHA"

# cli ref must be available
[ -n "${CLI_REF_SHA:-}" ] || fail "cli.ref missing"
echo "$CLI_REF_SHA" | grep -Eq '^[0-9a-f]{40}$' || fail "cli.ref invalid: $CLI_REF_SHA"

[ -d "$BIN_DIR" ] || fail "missing bin directory in stage: $BIN_DIR"
[ -f "$STAGE_DIR/bin/yai" ] || fail "manifest expects bin/yai but it's missing in stage"

required_bins=(yai-boot yai-root-server yai-kernel yai-engine yai)
for bin in "${required_bins[@]}"; do
  if [ ! -f "$BIN_DIR/$bin" ]; then
    fail "missing required binary for manifest: $BIN_DIR/$bin"
  fi
done

hash_file() {
  local f="$1"
  if command -v sha256sum >/dev/null 2>&1; then
    sha256sum "$f" | awk '{print $1}'
  elif command -v shasum >/dev/null 2>&1; then
    shasum -a 256 "$f" | awk '{print $1}'
  else
    fail "no sha256 tool found (sha256sum/shasum)"
  fi
}

file_size() {
  local f="$1"
  if stat -f%z "$f" >/dev/null 2>&1; then
    stat -f%z "$f"
  else
    stat -c%s "$f"
  fi
}

CREATED_AT="$(date -u +"%Y-%m-%dT%H:%M:%SZ")"

{
  printf '{\n'
  printf '  "bundle": {\n'
  printf '    "bundle_version": "%s",\n' "$BUNDLE_VERSION"
  printf '    "created_at": "%s",\n' "$CREATED_AT"
  printf '    "os": "%s",\n' "$PLATFORM_OS"
  printf '    "arch": "%s"\n' "$PLATFORM_ARCH"
  printf '  },\n'
  printf '  "core": {\n'
  printf '    "version": "%s",\n' "$CORE_VERSION"
  printf '    "git_sha": "%s"\n' "$CORE_GIT_SHA"
  printf '  },\n'
  printf '  "cli": {\n'
  printf '    "ref": "%s",\n' "$CLI_REF"
  printf '    "git_sha": "%s"\n' "$CLI_GIT_SHA"
  printf '  },\n'
  printf '  "specs": {\n'
  printf '    "path": "deps/yai-specs",\n'
  printf '    "git_sha": "%s"\n' "$SPECS_GIT_SHA"
  printf '  },\n'
  printf '  "artifacts": [\n'

  first=1
  while IFS= read -r file; do
    [ -f "$file" ] || continue
    rel="bin/$(basename "$file")"
    sha="$(hash_file "$file")"
    size="$(file_size "$file")"
    if [ "$first" -eq 0 ]; then
      printf ',\n'
    fi
    printf '    {"path": "%s", "sha256": "%s", "size": %s}' "$rel" "$sha" "$size"
    first=0
  done < <(find "$BIN_DIR" -maxdepth 1 -type f | sort)

  printf '\n  ]\n'
  printf '}\n'
} > "$OUT_MANIFEST"

for must in bin/yai bin/yai-boot bin/yai-kernel bin/yai-root-server bin/yai-engine; do
  grep -q "\"path\": \"$must\"" "$OUT_MANIFEST" || fail "manifest missing artifact entry: $must"
done

echo "Generated manifest: $OUT_MANIFEST"
