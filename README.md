# YAI

YAI is the core runtime in C for sovereign execution: boot, root plane, kernel, and engine.
Canonical contracts are pinned in `deps/yai-specs`.

## Core Contract

`deps/yai-specs` is the source of truth for protocol, control, graph, providers, vault, and formal contracts.
If runtime behavior diverges from `deps/yai-specs`, runtime is a bug and must be fixed.

## Repository Map

- `deps/yai-specs/` - L0 canonical contracts (submodule)
- `boot/` - runtime bootstrap and bring-up entrypoints
- `root/` - root plane services and control-plane core
- `kernel/` - L1 authority enforcement runtime
- `engine/` - L2 deterministic execution and provider gates
- `runtime/` - runtime glue and protocol integration surface
- `scripts/` - operations, verification, and gate scripts
- `docs/` - architecture, guides, runbooks, and reference docs
- `data/` - local datasets used by tests/ops; policy in `DATA_POLICY.md`

## Quickstart

```bash
git submodule update --init --recursive
make all
./build/bin/yai-boot
```

Verification commands available in this repo:

```bash
./scripts/yai-verify
./scripts/verify-core.sh
./scripts/verify-events.sh
```

## Ecosystem

- `yai-specs` - normative contracts repository
- `yai-cli` - operator/developer command-line client
- `yai-yx` - graphical runtime cockpit
- `yai-mind` - higher-level orchestration layer

## Build Outputs

- build: `build/bin/` (canonical compile outputs)
- dist: `dist/bin/` (staging copy from `build/bin/`)
- bundle: `dist/bundle/out/` (release assets)

Commands:

```bash
make all      # build only, no dist side-effects
make dist     # stage binaries to dist/bin
make bundle   # package tar/zip + manifest + checksums
```

## Distribution

GitHub Releases are the distribution channel for binaries and bundle metadata.
The repository remains source code only; runtime artifacts are produced via CI/local `make bundle`.

Verify downloaded bundles with `SHA256SUMS`:

```bash
# Linux
sha256sum -c yai-bundle-<version>-<os>-<arch>.SHA256SUMS

# macOS
shasum -a 256 -c yai-bundle-<version>-<os>-<arch>.SHA256SUMS
```

## License

Apache License 2.0 (Apache-2.0).
See `LICENSE` and `NOTICE`.
Third-party licensing notices are tracked in `THIRD_PARTY_NOTICES.md`.
Datasets may have separate terms if stated in-place.
