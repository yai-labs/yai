# SC-yai-core

## 1) Health

### Claim
Core build and formal baseline are executable.

### Evidence
- `yai/.github/workflows/ci.yml`
- `yai/tools/ops/verify/core.sh`
- Local run (2026-02-19): `make all` passed; `tools/bin/yai-verify core` passed (TLC quick+deep + build).

### Confidence
High

### Gaps
- CI mainly runs build/generated checks; runtime behavior gates are not fully enforced in CI non-skip mode.

## 2) Contract alignment

### Claim
Core explicitly pins and validates specs, with release-train pin checks.

### Evidence
- `yai/.gitmodules` (`deps/yai-specs`)
- `yai/tools/release/check_pins.sh`
- `yai/deps/yai-cli.ref`
- `yai/COMPATIBILITY.md`, `yai/VERSIONING.md`

### Confidence
High

### Drift risk
Medium (runtime docs/gates can drift from actual CLI/runtime command surfaces).

## 3) Operational readiness

### Claim
Operational framework is broad, but proof depth is uneven due skip pathways.

### Evidence
- Gate/suite tooling: `tools/ops/gate/*`, `tools/ops/suite/*`, `tools/ops/verify/*`
- Local run (2026-02-19): `./tools/ops/suite/levels/l0-l7.sh` reports OK with multiple `SKIP` at L3-L7.

### Confidence
Medium

### Gaps
- “Pass with skip” weakens TRL evidence quality.
- Some runbook tracks are planning-heavy with limited closed phase evidence.

## 4) Critical gaps (top 3)

1. Introduce strict TRL profile where required gates fail (not skip) when capability missing.
2. Align runtime gate expectations with currently shipped CLI command surface and enforce via CI.
3. Close root/workspace/engine phase evidence with artifacted positive+negative deterministic outputs.
