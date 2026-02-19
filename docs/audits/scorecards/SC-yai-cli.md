# SC-yai-cli

## 1) Health

### Claim
Repo-level verify flow is functional and reproducible.

### Evidence
- `yai-cli/.github/workflows/ci.yml`
- `yai-cli/tools/bin/yai-cli-verify`
- Local run (2026-02-19): `./tools/bin/yai-cli-verify --profile ci` passed (layout/specs/build/tests).

### Confidence
High

### Gaps
- Test depth is limited (unit parse placeholder and minimal vector checks).

## 2) Contract alignment

### Claim
Pinning process exists, but behavior drifts from command contract currently pinned.

### Evidence
- Contract source: `yai-cli/deps/yai-specs/specs/cli/schema/commands.v1.json` (includes lifecycle commands like `up/down/status`).
- Implementation entry: `yai-cli/src/cli/dispatch.c` (plane-command dispatcher).
- Local run (2026-02-19): `./dist/bin/yai-cli up --help` returns unknown target.

### Confidence
High

### Drift risk
High

## 3) Operational readiness

### Claim
Tooling/release hygiene exists, but docs completeness for build/release/testing is partial.

### Evidence
- `docs/development/specs-pinning.md`
- `docs/reference/contract.md`
- TODO placeholders: `docs/development/build.md`, `docs/development/release.md`

### Confidence
Medium

### Gaps
- Missing complete maintainers-grade build/release docs.
- Conformance checks currently ensure pin cleanliness, not full command-behavior conformance.

## 4) Critical gaps (top 3)

1. Reconcile `commands.v1.json` with actual CLI surface (spec update or implementation update).
2. Expand tests to include deterministic negative vectors mapped to protocol errors/roles/arming.
3. Add CI check that validates exported command tree against pinned command schema.
