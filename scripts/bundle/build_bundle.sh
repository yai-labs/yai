#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
DIST_ROOT="${DIST_ROOT:-$ROOT_DIR/dist}"
BIN_DIST="${BIN_DIST:-$DIST_ROOT/bin}"
BUNDLE_ROOT="$DIST_ROOT/bundle"
STAGE_ROOT="$BUNDLE_ROOT/stage"
OUT_ROOT="$BUNDLE_ROOT/out"
TMP_ROOT="$BUNDLE_ROOT/.bundle_tmp"
CLI_PIN_FILE="$ROOT_DIR/deps/yai-cli.ref"

YAI_CLI_REPO="${YAI_CLI_REPO:-https://github.com/francescomaiomascio/yai-cli.git}"

EXPECTED_BINS=(
  yai-boot
  yai-root-server
  yai-kernel
  yai-engine
)

if [ ! -d "$BIN_DIST" ]; then
  echo "ERROR: missing $BIN_DIST. Run 'make dist' first." >&2
  exit 1
fi

for bin in "${EXPECTED_BINS[@]}"; do
  if [ ! -f "$BIN_DIST/$bin" ]; then
    echo "ERROR: missing dist binary $BIN_DIST/$bin. Run 'make dist'." >&2
    exit 1
  fi
done

if [ ! -d "$ROOT_DIR/deps/yai-specs" ]; then
  echo "ERROR: missing deps/yai-specs. Run 'git submodule update --init --recursive'." >&2
  exit 1
fi

if [ ! -f "$CLI_PIN_FILE" ]; then
  echo "ERROR: missing CLI pin file $CLI_PIN_FILE" >&2
  exit 1
fi
CLI_PIN_SHA="$(awk -F= '/^cli_sha=/{print $2}' "$CLI_PIN_FILE" | tr -d '[:space:]')"
if ! echo "$CLI_PIN_SHA" | grep -Eq '^[0-9a-f]{40}$'; then
  echo "ERROR: invalid cli_sha in $CLI_PIN_FILE (expected 40-hex SHA)" >&2
  exit 1
fi

CORE_GIT_SHA="$(git -C "$ROOT_DIR" rev-parse HEAD)"
CORE_GIT_SHA_SHORT="$(git -C "$ROOT_DIR" rev-parse --short=12 HEAD)"
CORE_VERSION_RAW="$(git -C "$ROOT_DIR" describe --tags --exact-match 2>/dev/null || true)"
if [ -z "$CORE_VERSION_RAW" ] && [ -f "$ROOT_DIR/VERSION" ]; then
  CORE_VERSION_RAW="v$(tr -d '[:space:]' < "$ROOT_DIR/VERSION")"
fi
if [ -z "$CORE_VERSION_RAW" ]; then
  CORE_VERSION_RAW="dev-$(date -u +%Y%m%d)-$CORE_GIT_SHA_SHORT"
fi
CORE_VERSION="${CORE_VERSION_RAW#v}"

UNAME_S="$(uname -s)"
case "$UNAME_S" in
  Linux) PLATFORM_OS="linux" ;;
  Darwin) PLATFORM_OS="macos" ;;
  *) PLATFORM_OS="$(echo "$UNAME_S" | tr '[:upper:]' '[:lower:]')" ;;
esac
PLATFORM_ARCH="$(uname -m)"

BUNDLE_NAME_TMP="yai-${CORE_VERSION}-${PLATFORM_OS}-${PLATFORM_ARCH}"
STAGE_DIR="$STAGE_ROOT/$BUNDLE_NAME_TMP"

rm -rf "$STAGE_DIR" "$TMP_ROOT"
mkdir -p "$STAGE_DIR/bin" "$OUT_ROOT" "$TMP_ROOT"

for bin in "${EXPECTED_BINS[@]}"; do
  cp "$BIN_DIST/$bin" "$STAGE_DIR/bin/$bin"
done

# -----------------------------
# CLI ingest (hard-fail on any error)
# -----------------------------
CLI_SRC_DIR="$TMP_ROOT/yai-cli"
echo "[bundle] ingesting yai-cli from $YAI_CLI_REPO @ $CLI_PIN_SHA"
git clone "$YAI_CLI_REPO" "$CLI_SRC_DIR"
if ! git -C "$CLI_SRC_DIR" checkout "$CLI_PIN_SHA"; then
  echo "ERROR: failed to checkout pinned yai-cli SHA $CLI_PIN_SHA" >&2
  exit 1
fi

# Force specs parity with this runtime bundle pin to avoid drift.
rm -rf "$CLI_SRC_DIR/deps/yai-specs"
mkdir -p "$CLI_SRC_DIR/deps/yai-specs"
(
  cd "$ROOT_DIR/deps/yai-specs"
  tar --exclude='.git' -cf - .
) | (
  cd "$CLI_SRC_DIR/deps/yai-specs"
  tar -xf -
)

# Build only the CLI executable target (skip docs side effects).
make -C "$CLI_SRC_DIR" "$CLI_SRC_DIR/dist/bin/yai-cli"
CLI_BIN="$CLI_SRC_DIR/dist/bin/yai-cli"
if [ ! -f "$CLI_BIN" ]; then
  echo "ERROR: yai-cli build succeeded but binary not found at $CLI_BIN" >&2
  exit 1
fi
cp "$CLI_BIN" "$STAGE_DIR/bin/yai"
chmod +x "$STAGE_DIR/bin/yai"

CLI_GIT_SHA="$(git -C "$CLI_SRC_DIR" rev-parse HEAD)"
CLI_GIT_SHA_SHORT="$(git -C "$CLI_SRC_DIR" rev-parse --short=12 HEAD)"
if [ "$CLI_GIT_SHA" != "$CLI_PIN_SHA" ]; then
  echo "ERROR: checked out CLI SHA ($CLI_GIT_SHA) does not match pinned SHA ($CLI_PIN_SHA)" >&2
  exit 1
fi

if [ -n "${BUNDLE_VERSION:-}" ]; then
  VERSION="$BUNDLE_VERSION"
else
  VERSION="${CORE_VERSION}+cli.${CLI_GIT_SHA_SHORT}"
fi

BUNDLE_NAME="yai-${VERSION}-${PLATFORM_OS}-${PLATFORM_ARCH}"
FINAL_STAGE_DIR="$STAGE_ROOT/$BUNDLE_NAME"
rm -rf "$FINAL_STAGE_DIR"
mv "$STAGE_DIR" "$FINAL_STAGE_DIR"
STAGE_DIR="$FINAL_STAGE_DIR"

mkdir -p "$STAGE_DIR/specs"
(
  cd "$ROOT_DIR/deps/yai-specs"
  tar --exclude='.git' -cf - .
) | (
  cd "$STAGE_DIR/specs"
  tar -xf -
)

for f in LICENSE NOTICE THIRD_PARTY_NOTICES.md DATA_POLICY.md; do
  if [ -f "$ROOT_DIR/$f" ]; then
    cp "$ROOT_DIR/$f" "$STAGE_DIR/$f"
  fi
done

cat > "$STAGE_DIR/INSTALL.md" <<BUNDLE_INSTALL
# YAI Bundle Install

1. \`export PATH="$(pwd)/bin:$PATH"\`
2. \`./bin/yai --help\`

Optional:
- \`./bin/yai status\`
- \`./bin/yai-boot\`
BUNDLE_INSTALL

SPECS_COMMIT="$(git -C "$ROOT_DIR" submodule status -- deps/yai-specs | awk '{print $1}' | sed 's/^[-+]//')"
if [ -z "$SPECS_COMMIT" ]; then
  SPECS_COMMIT="unknown"
fi

bash "$ROOT_DIR/scripts/bundle/manifest.sh" \
  "$STAGE_DIR" \
  "$VERSION" \
  "$CORE_VERSION" \
  "$CORE_GIT_SHA" \
  "$CLI_PIN_SHA" \
  "$CLI_GIT_SHA" \
  "$SPECS_COMMIT" \
  "$PLATFORM_OS" \
  "$PLATFORM_ARCH"

(
  cd "$STAGE_DIR"
  if command -v sha256sum >/dev/null 2>&1; then
    find . -type f ! -name SHA256SUMS -print | sort | sed 's#^./##' | while read -r f; do
      sha256sum "$f"
    done > SHA256SUMS
  elif command -v shasum >/dev/null 2>&1; then
    find . -type f ! -name SHA256SUMS -print | sort | sed 's#^./##' | while read -r f; do
      shasum -a 256 "$f"
    done > SHA256SUMS
  else
    echo "ERROR: sha256 tool not found (sha256sum/shasum)." >&2
    exit 1
  fi
)

TAR_OUT="$OUT_ROOT/${BUNDLE_NAME}.tar.gz"
ZIP_OUT="$OUT_ROOT/${BUNDLE_NAME}.zip"
MANIFEST_OUT="$OUT_ROOT/${BUNDLE_NAME}.manifest.json"
SHA_OUT="$OUT_ROOT/${BUNDLE_NAME}.SHA256SUMS"

tar -C "$STAGE_ROOT" -czf "$TAR_OUT" "$BUNDLE_NAME"
(
  cd "$STAGE_ROOT"
  zip -qr "$ZIP_OUT" "$BUNDLE_NAME"
)

cp "$STAGE_DIR/manifest.json" "$MANIFEST_OUT"
cp "$STAGE_DIR/SHA256SUMS" "$SHA_OUT"

if [ ! -x "$STAGE_DIR/bin/yai" ]; then
  echo "ERROR: bundled CLI is not executable at $STAGE_DIR/bin/yai" >&2
  exit 1
fi

rm -rf "$TMP_ROOT"

echo "Bundle output:"
echo "  $TAR_OUT"
echo "  $ZIP_OUT"
echo "  $MANIFEST_OUT"
echo "  $SHA_OUT"
