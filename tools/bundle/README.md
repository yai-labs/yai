# Bundle scripts

- `build_bundle.sh`: builds release assets from `dist/bin` into `dist/bundle/out`.
- `manifest.sh`: generates `manifest.json` with bundle metadata and binary checksums.

## Usage

```bash
make dist
make bundle
```

Optional version override:

```bash
BUNDLE_VERSION=v0.1.0 make bundle
```
