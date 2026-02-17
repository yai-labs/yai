# Releases

1. Ensure `main` is green in CI.
2. Create and push a semantic tag: `git tag vX.Y.Z && git push origin vX.Y.Z`.
3. Tag push triggers `.github/workflows/bundle.yml`.
4. CI runs `make bundle` on Ubuntu and macOS.
5. Each job uploads `dist/bundle/out/*` as workflow artifacts.
6. On tag events, CI creates a GitHub Release and attaches all assets.

## Bundle outputs

- `yai-bundle-<version>-<os>-<arch>.tar.gz`
- `yai-bundle-<version>-<os>-<arch>.zip`
- `yai-bundle-<version>-<os>-<arch>.manifest.json`
- `yai-bundle-<version>-<os>-<arch>.SHA256SUMS`

## Verify hashes

- Linux: `sha256sum -c yai-bundle-<version>-<os>-<arch>.SHA256SUMS`
- macOS: `shasum -a 256 -c yai-bundle-<version>-<os>-<arch>.SHA256SUMS`

Releases are distribution artifacts. Source code remains in the repository; runtime binaries are consumed from release bundles.
