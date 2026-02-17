# Compatibility Matrix

This matrix defines baseline compatibility for the `yai` runtime.

## Runtime and Specs

| yai | SPECS_API_VERSION | deps/yai-specs pin | Notes |
|---|---|---|---|
| v0.1.x | v1 | pinned submodule commit in `deps/yai-specs` | initial public baseline |

## Platform Support (Current)

| Platform | Status | CI Coverage | Notes |
|---|---|---|
| Ubuntu 22.04 | Supported | `ci.yml` | full build + generated artifact checks |
| Ubuntu latest | Supported | `ci.yml` | full build + generated artifact checks |
| Arch Linux (latest) | Supported | `ci.yml` (container) | full build + generated artifact checks |
| Debian stable | Supported | `ci.yml` (container) | full build + generated artifact checks |
| Fedora latest | Supported | `ci.yml` (container) | full build + generated artifact checks |
| macOS 13 | Supported | `ci.yml` | build + generated artifact checks |
| macOS latest | Supported | `ci.yml` | build + generated artifact checks |
| Windows | Not officially supported | none | no compatibility guarantees yet |

## Toolchain Baseline

| Tool | Requirement |
|---|---|
| C compiler | gcc or clang |
| Build | GNU make |
| Python | Python 3 (scripts/verification helpers) |

## Notes

- Runtime behavior is contract-bound to `deps/yai-specs`.
- If implementation and specs diverge, specs are authoritative.

## License

This policy is part of the Apache-2.0 licensed repository. See `LICENSE` and `NOTICE`.
