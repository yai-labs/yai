# Changelog

All notable changes to this project are documented here.

Format: Keep a Changelog.
Versioning: Semantic Versioning.

## [Unreleased]

### Added
- `tools/release/unified_repo_convergence_smoke.sh` for final single-repo convergence smoke checks.
- `tools/validate/validate_root_framing.py` for root-level canonical framing checks.
- `tools/bin/yai-governance-compat-check` canonical governance compatibility check entrypoint.

### Changed
- Finalized single-repo convergence framing in root policy docs (`README.md`, `FOUNDATION.md`, `GOVERNANCE.md`, `COMPATIBILITY.md`, `VERSIONING.md`).
- Completed embedded shutdown and removed active `governance/runtime-package` architecture surface.
- Eliminated legacy sync/tooling assumptions (`../governance`, `YAI_GOVERNANCE_*`, legacy resolver scripts) from canonical wrappers and active test/tool flows.
- Canonicalized publish target naming to `runtime-governance`.

### Removed
- `governance/runtime-package/**` from active repository topology.
- legacy resolver scripts: `tools/dev/resolve-governance-compat.sh`, `tools/dev/resolve-governance-embed.sh`.

## [1.0.0] - 2026-03-11

### Added
- Unified repository baseline declared operationally self-sufficient.

### Changed
- Governance moved to canonical in-repo source-of-truth model.
- Release/compatibility narrative converged to single-repo ownership.

## [0.1.7] - 2026-02-17

- Maintenance and hardening prior to unified convergence.

## [0.1.6] - 2026-02-17

- Maintenance and hardening prior to unified convergence.

## [0.1.5] - 2026-02-17

- Maintenance and hardening prior to unified convergence.

## [0.1.4] - 2026-02-17

- Maintenance and hardening prior to unified convergence.

## [0.1.3] - 2026-02-17

- Maintenance and hardening prior to unified convergence.

## [0.1.2] - 2026-02-17

- Maintenance and hardening prior to unified convergence.

## [0.1.1] - 2026-02-17

- Maintenance and hardening prior to unified convergence.

## [0.1.0] - 2026-02-17

### Added

- Canonical `build -> dist -> bundle` pipeline with reproducible release assets.

### Changed

- Runtime build outputs standardized under `build/bin`.

### Security

- Public disclosure process and hardening checklist documented.

## License

Apache-2.0.
