---
id: MP-CONTRACT-BASELINE-LOCK-0.1.2
status: draft
runbook: docs/20-program/23-runbooks/contract-baseline-lock.md
phase: "0.1.2 — No Pass-on-Skip Enforcement"
adrs:
  - docs/20-program/22-adr/ADR-011-contract-baseline-lock.md
  - docs/20-program/22-adr/ADR-012-audit-convergence-gates.md
spec_anchors:
  - deps/yai-specs/contracts/invariants/I-001-traceability.md
  - deps/yai-specs/contracts/invariants/I-003-governance.md
claims:
  - C-SKIP-FAIL-MANDATORY
  - C-EVIDENCE-PACK-REPRODUCIBLE
evidence_commands_required:
  - tools/bin/yai-proof-check
  - tools/bin/yai-docs-trace-check --all
issues:
  - 141
---
# MP-CONTRACT-BASELINE-LOCK-0.1.2

## Metadata

- Runbook: `docs/20-program/23-runbooks/contract-baseline-lock.md`
- Phase: `0.1.2 — No Pass-on-Skip Enforcement`
- Wave issue: `#141`
- Owner: `governance`
- Status: `draft`

## Links

- ADR: `docs/20-program/22-adr/ADR-011-contract-baseline-lock.md`
- ADR: `docs/20-program/22-adr/ADR-012-audit-convergence-gates.md`
- Proposal: `docs/20-program/21-rfc/RFC-004-contract-baseline-lock-and-pin-policy.md`
- Evidence plans: `docs/40-qualification/test-plans/hardfail.md`
- Claims registry: `docs/50-validation/audits/claims/infra-grammar.v0.1.json`

Objective:
- Prevent mandatory proof and contract checks from passing through skip paths.

Contract Delta:
- Envelope: none.
- Authority: none.
- Errors: skipped mandatory checks must return failure.
- Logging: CI logs must expose skipped mandatory checks as failures.

Repo Split:
- `yai`: mandatory checks configured as hard-fail when skipped.
- `yai-cli`: mandatory checks configured as hard-fail when skipped.

Evidence Plan (minimum):
- Positive cases:
  - Mandatory checks execute and pass when all prerequisites exist.
  - Governance checks report complete mandatory coverage.
- Negative cases:
  - Simulated skip of required check fails pipeline.
  - Missing mandatory evidence blocks merge.

Mandatory command outcomes:
- `tools/bin/yai-proof-check` -> `PASS`
- `tools/bin/yai-docs-trace-check --all` -> `PASS`

Closure policy:
- mandatory command `SKIP` is treated as `FAIL`.

Compatibility Classification:
- Type: A
- Rationale: governance policy hardening without API/protocol change.
- Upgrade path: no runtime migration required.

Definition of Done:
- [ ] Mandatory proof checks cannot pass via skip.
- [ ] Pipeline fails deterministically on missing mandatory evidence.
- [ ] Failure semantics are documented and auditable.
- [ ] Runbook/ADR references are present in closure evidence.
- [ ] Mandatory command outcomes are recorded as `PASS` (no `SKIP` closure).

## Execution Snapshot (2026-02-21)

- Evidence bundle: `docs/20-program/24-milestone-packs/contract-baseline-lock/evidence/wave0-2026-02-21/`
- `tools/bin/yai-proof-check` -> `PASS` (`exit=0`) via `07-proof-check-public.*`
- `tools/bin/yai-proof-check --manifest docs/50-validation/proof/.private/PP-FOUNDATION-0001/pp-foundation-0001.manifest.v1.json` -> `FAIL` (`exit=2`) via `08-proof-check-private-hardfail.*`
- `tools/bin/yai-docs-trace-check --all` -> `PASS` (`exit=0`)

Phase state:
- `READY` for phase closure (no mandatory `SKIP` path on proof-check).

Wave-level status:
- strict pin alignment now passing (`tools/bin/yai-check-pins` -> `PASS`) via `10-check_pins-strict-pass.*`.
