# Releases

Official distribution for this repository is a single Runtime Bundle:

- core runtime binaries (`yai-boot`, `yai-root-server`, `yai-kernel`, `yai-engine`)
- `yai` CLI binary (built from `yai-cli`)
- pinned `specs/` snapshot
- `manifest.json` with core/cli/specs metadata and per-binary SHA256

Core-only artifacts are internal and are not the user-facing release contract.

## Bundle outputs

- `yai-<bundle_version>-<os>-<arch>.tar.gz`
- `yai-<bundle_version>-<os>-<arch>.zip`
- `yai-<bundle_version>-<os>-<arch>.manifest.json`
- `yai-<bundle_version>-<os>-<arch>.SHA256SUMS`

## Install (minimal)

1. Extract the archive.
2. Run:

```bash
export PATH="$(pwd)/bin:$PATH"
./bin/yai --help
```

Optional runtime entry:

```bash
./bin/yai-boot
```

## Verify hashes

- Linux: `sha256sum -c yai-<bundle_version>-<os>-<arch>.SHA256SUMS`
- macOS: `shasum -a 256 -c yai-<bundle_version>-<os>-<arch>.SHA256SUMS`

## Release flow

1. Ensure `main` is green.
2. Create and push a semantic tag: `git tag vX.Y.Z && git push origin vX.Y.Z`.
3. Tag push triggers `.github/workflows/bundle.yml`.
4. CI runs `make bundle` on Ubuntu and macOS.
5. CI uploads bundle assets to the GitHub Release for that tag.

Compatibility contract:
- CLI and runtime/specs are shipped together by design.
- If `manifest.json` is missing or invalid, or if CLI/runtime/specs metadata mismatches, treat the bundle as invalid and download the official release artifact again.
