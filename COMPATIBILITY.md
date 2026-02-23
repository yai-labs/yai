# Compatibility

This document defines compatibility guarantees for the `yai` runtime repository.

Compatibility in YAI is contract-driven: normative behavior is defined by pinned specs, and this runtime is required to remain aligned with them.

## Contract Compatibility

### Specs API Baseline

| `yai` line | Specs API | `deps/yai-specs` pin policy | Guarantee |
|---|---|---|---|
| `v0.1.x` | `v1` | pinned submodule commit under `deps/yai-specs` | Spec compliance is required; drift is a defect |

### Contract Surfaces

Contract-facing behavior is governed by `deps/yai-specs` and includes (non-exhaustive):

- protocol envelope and roles
- control plane authority surfaces
- vault ABI / shared memory surfaces
- graph and provider surfaces (where applicable)
- compliance packs and schemas (where applicable)

If implementation conflicts with specs, **specs are authoritative**.

## Platform Compatibility

YAI is validated on a defined platform set. “Supported” means: CI covers the platform, and breakages are treated as defects on the active development line.

| Platform | Support level | CI coverage | Notes |
|---|---|---|---|
| Ubuntu 22.04 | Supported | `ci.yml` | full build + verification gates |
| Ubuntu latest | Supported | `ci.yml` | full build + verification gates |
| Arch Linux (latest) | Supported | `ci.yml` (container) | full build + verification gates |
| Debian stable | Supported | `ci.yml` (container) | full build + verification gates |
| Fedora latest | Supported | `ci.yml` (container) | full build + verification gates |
| macOS latest | Supported | `ci.yml` | build + verification gates |
| Windows | Not supported | none | no guarantees; may build opportunistically |

Notes:
- “CI coverage” refers to workflows that execute build/verify gates on pull requests.
- Local developer environments outside this matrix are “best effort”.

## Toolchain Baseline

Minimum expected toolchain for contributors:

| Tool | Requirement |
|---|---|
| C compiler | `gcc` or `clang` |
| Build system | `make` |
| Python | Python 3 (required for verification/governance tooling) |
| Rust | required only when building or modifying `mind/` |

## Compatibility Rules

1. **Pinned specs are part of compatibility.** Updating `deps/yai-specs` changes the contract baseline and must be justified (versioning + evidence).
2. **No silent drift.** Runtime changes that alter contract-facing behavior without a corresponding specs change are not accepted.
3. **Breaking changes require explicit handling.** If a change is breaking, it must be reflected in:
   - `VERSIONING.md` policy
   - `CHANGELOG.md` entry under Unreleased
   - evidence showing intentional compatibility impact and migration path (if applicable)

## License

This policy is part of the Apache-2.0 licensed repository. See `LICENSE` and `NOTICE`.