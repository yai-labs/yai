# Tooling

## Toolchain Contract
Canonical governance/tooling contracts are maintained in `infra`; this repo consumes them.

## Repository Tooling Surfaces
- Wrappers and entrypoints: `tools/bin/`
- Validation scripts: `tools/validate/`
- Release/smoke scripts: `tools/release/`

## Documentation Integration
When command or tooling entrypoints change, update:
- `docs/reference/commands/`
- `docs/runbooks/`
- `docs/guides/developer/build-test/`
