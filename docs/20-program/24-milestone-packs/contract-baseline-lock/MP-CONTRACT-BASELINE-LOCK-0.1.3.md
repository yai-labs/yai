---
id: MP-CONTRACT-BASELINE-LOCK-0.1.3
status: draft
runbook: docs/20-program/23-runbooks/contract-baseline-lock.md
phase: "0.1.3 — Formal/Core Sync on Contract Delta"
adrs:
  - docs/20-program/22-adr/ADR-011-contract-baseline-lock.md
  - docs/20-program/22-adr/ADR-012-audit-convergence-gates.md
spec_anchors:
  - deps/yai-specs/contracts/invariants/I-002-determinism.md
  - deps/yai-specs/contracts/invariants/I-003-governance.md
  - deps/yai-specs/contracts/invariants/I-006-external-effect-boundary.md
claims:
  - C-AUTHORITY-SURFACE-RUNTIME
  - C-EVIDENCE-PACK-REPRODUCIBLE
evidence_commands_required:
  - tools/bin/yai-proof-check
  - tools/bin/yai-verify
issues:
  - 141
---
# MP-CONTRACT-BASELINE-LOCK-0.1.3

## Metadata

- Runbook: `docs/20-program/23-runbooks/contract-baseline-lock.md`
- Phase: `0.1.3 — Formal/Core Sync on Contract Delta`
- Wave issue: `#141`
- Owner: `governance`
- Status: `draft`

## Links

- ADR: `docs/20-program/22-adr/ADR-011-contract-baseline-lock.md`
- ADR: `docs/20-program/22-adr/ADR-012-audit-convergence-gates.md`
- Proposal: `docs/20-program/21-rfc/RFC-005-formal-coverage-roadmap.md`
- Evidence plans: `docs/40-qualification/test-plans/hardfail.md`
- Claims registry: `docs/50-validation/audits/claims/infra-grammar.v0.1.json`

Objective:
- Require formal/core verification updates whenever authority or envelope contracts change.

Contract Delta:
- Envelope: contract delta requires corresponding verification delta.
- Authority: contract delta requires corresponding verification delta.
- Errors: merges without required verification updates must fail.
- Logging: traceability evidence must link contract change to verification change.

Repo Split:
- `yai`: core verification artifacts updated for contract deltas.
- `yai-cli`: contract/vector verification updated when CLI surface is impacted.

Evidence Plan (minimum):
- Positive cases:
  - Contract delta with matching verify updates passes all gates.
  - Traceability graph links contract and verify changes.
- Negative cases:
  - Contract delta without verify updates is blocked.
  - Partial verify update without matching contract scope is blocked.

Mandatory command outcomes:
- `tools/bin/yai-proof-check` -> `PASS`
- `tools/bin/yai-verify` -> `PASS`

Closure policy:
- mandatory command `SKIP` is treated as `FAIL`.

Compatibility Classification:
- Type: B
- Rationale: introduces stricter merge constraints bound to verification obligations.
- Upgrade path: contributors must include synchronized verify deltas when changing contracts.

Definition of Done:
- [ ] Contract deltas trigger required formal/core sync checks.
- [ ] Unsynced deltas are blocked deterministically.
- [ ] Evidence shows explicit contract-to-verify mapping.
- [ ] All links are traceable in docs and CI output.
- [ ] Mandatory command outcomes are recorded as `PASS` (no `SKIP` closure).

## Execution Snapshot (2026-02-21)

- Evidence bundle: `docs/20-program/24-milestone-packs/contract-baseline-lock/evidence/wave0-2026-02-21/`
- `tools/bin/yai-proof-check` -> `PASS` (`exit=0`) via `07-proof-check-public.*`
- `tools/bin/yai-proof-check --manifest docs/50-validation/proof/.private/PP-FOUNDATION-0001/pp-foundation-0001.manifest.v1.json` -> `FAIL` (`exit=2`) via `08-proof-check-private-hardfail.*`
- `tools/bin/yai-verify core` -> `PASS` (`exit=0`)
- `tools/bin/yai-verify law-kernel` -> `PASS` (`exit=0`)

Phase state:
- `READY` for phase closure (mandatory checks are non-skip and passing).

Wave-level status:
- strict pin alignment now passing (`tools/bin/yai-check-pins` -> `PASS`) via `10-check_pins-strict-pass.*`.
