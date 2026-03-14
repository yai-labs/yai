# Testing

## Verification Commands
- `tools/bin/yai-docs-trace-check --all`
- `./tools/bin/yai-governance-compat-check`
- `./tools/release/unified_repo_convergence_smoke.sh`

## Repository Test Surfaces
- Integration tests: `tests/integration/`
- Unit tests: `tests/legacy/unit/`
- Runtime qualification artifacts: `docs/runbooks/qualification/`

## Rules
- Prefer deterministic tests with local fixtures.
- Record unavailable commands explicitly in report evidence.
