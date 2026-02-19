# Changelog

All notable changes to this project will be documented in this file.

The format is based on Keep a Changelog,
and this project adheres to Semantic Versioning.

## [Unreleased]

### Added

- Introduced `yai-changelog-check` with PR-mode and tag-mode validation for changelog quality gates.
- Added CI workflow `validate-changelog.yml` to enforce incremental changelog policy on pull requests.

### Changed

- Release workflow now runs strict changelog validation in tag mode before publishing bundle assets.
- `tools/release/bump_version.sh` no longer injects automatic placeholder entries.

## [0.1.7] - 2026-02-17

- TODO: summarize release changes.

## [0.1.6] - 2026-02-17

- TODO: summarize release changes.

## [0.1.5] - 2026-02-17

- TODO: summarize release changes.

## [0.1.4] - 2026-02-17

- TODO: summarize release changes.

## [0.1.3] - 2026-02-17

- TODO: summarize release changes.

## [0.1.2] - 2026-02-17

- TODO: summarize release changes.

## [0.1.1] - 2026-02-17

- TODO: summarize release changes.

- No unreleased changes.

## [0.1.0] - 2026-02-17

### Added

- Canonical `build/ -> dist/ -> bundle/` pipeline with reproducible release assets.
- Release bundle artifacts (`tar.gz`, `zip`, `manifest`, `SHA256SUMS`) and CI workflow.

### Changed

- Repository legal/governance hardening for public release readiness.
- Runtime build outputs standardized under `build/bin`.

### Security

- Public disclosure process and hardening checklist documented.

## License

This changelog is part of the Apache-2.0 licensed repository. See `LICENSE` and `NOTICE`.
