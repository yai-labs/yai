# Repo Cleanup — Mind Post Migration

Date: 2026-03-07

## Scope

This cleanup pass targets repository hygiene after Mind Rust->C migration.
`deps/**` is explicitly out of scope.

## 1. Artifacts cleaned

- Generated Mind artifacts removed from working tree:
  - `mind/build/`
  - `mind/dist/`
- Existing top-level generated outputs remain ignored (`build/`, `target/`, logs, objects).

## 2. Ignore rules

Existing root `.gitignore` already covers required patterns:

- `build/`
- `dist/`
- `*.o`
- `*.d`
- `*.log`
- `target/`

No extra broad ignore rule was required for this pass.

## 3. Documentation updates

Updated to reflect Mind C canonical path:

- `docs/guides/developer/build-test/build.md`
- `docs/guides/developer/build-test/mind-build.md`
- `docs/guides/developer/build-test/mind-testing.md`
- `docs/guides/user/user-guide/activation.md`

Historical runbook marked as non-canonical for active runtime usage:

- `docs/program/milestone-packs/runtime-baselines/mind-redis-stm.md`

## 4. Intentionally retained references

- Migration phase reports under `mind/docs/` and `mind/docs/archive/` are retained for traceability.
- They are historical, not operational build authority.

## 5. Out-of-scope references under deps

All `deps/**` references were intentionally ignored in this cleanup pass.

## 6. Residuals outside scope

- Existing root-level Rust workspace files (`Cargo.toml`, `Cargo.lock`) are currently removed in this migration branch state; decision for final retention/removal is tied to overall repo migration policy, not this cleanup pass alone.

## 7. Verification

Run after cleanup:

- `make -C mind`
- `make -C mind test`
