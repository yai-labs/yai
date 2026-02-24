---
id: MP-CONTRACT-BASELINE-LOCK-0.1.4
status: active
runbook: docs/20-program/23-runbooks/contract-baseline-lock.md
phase: "0.1.4 — Cross-Repo Evidence Closure"
adrs:
  - docs/20-program/22-adr/ADR-011-contract-baseline-lock.md
  - docs/20-program/22-adr/ADR-012-audit-convergence-gates.md
spec_anchors:
  - deps/yai-law/contracts/invariants/I-001-traceability.md
  - deps/yai-law/contracts/invariants/I-003-governance.md
  - deps/yai-law/contracts/boundaries/L1-kernel.md
claims:
  - C-EVIDENCE-PACK-REPRODUCIBLE
  - C-SPEC-FIRST-PINNED
evidence_commands_required:
  - tools/bin/yai-check-pins
  - tools/bin/yai-docs-trace-check --all
  - tools/bin/yai-proof-check
issues:
  - 141
---
# MP-CONTRACT-BASELINE-LOCK-0.1.4

## Metadata

- Runbook: `docs/20-program/23-runbooks/contract-baseline-lock.md`
- Phase: `0.1.4 — Cross-Repo Evidence Closure`
- Wave issue: `#141`
- Owner: `governance`
- Status: `active`

## Links

- ADR: `docs/20-program/22-adr/ADR-011-contract-baseline-lock.md`
- ADR: `docs/20-program/22-adr/ADR-012-audit-convergence-gates.md`
- Proposals: `docs/20-program/21-rfc/RFC-004-contract-baseline-lock-and-pin-policy.md`, `docs/20-program/21-rfc/RFC-005-formal-coverage-roadmap.md`
- Evidence plans: `docs/40-qualification/test-plans/hardfail.md`
- Claims registry: `docs/50-validation/audits/claims/infra-grammar.v0.1.json`

Objective:
- Close Milestone 1 with explicit, auditable cross-repo evidence for the baseline lock track.

Contract Delta:
- Envelope: none.
- Authority: none.
- Errors: closure fails if evidence set is incomplete.
- Logging: evidence index must include reproducible command/log pointers.

Repo Split:
- `yai-law`: normative contract and governance anchors.
- `yai`: consumer pin/proof evidence and gate results.
- `yai-cli`: consumer pin/proof evidence and gate results.

Evidence Plan (minimum):
- Positive cases:
  - Evidence bundle covers all completed phases `0.1.0` to `0.1.4`.
  - Cross-repo references are complete and machine-navigable.
- Negative cases:
  - Missing repo evidence blocks closure.
  - Untraceable evidence pointers block closure.

Mandatory command outcomes:
- `tools/bin/yai-check-pins` -> `PASS`
- `tools/bin/yai-docs-trace-check --all` -> `PASS`
- `tools/bin/yai-proof-check` -> `PASS`

Closure policy:
- mandatory command `SKIP` is treated as `FAIL`.

Compatibility Classification:
- Type: A
- Rationale: closure/evidence hardening only; no protocol behavior change.
- Upgrade path: none required for runtime clients.

Definition of Done:
- [x] Evidence index spans `yai-law`, `yai`, and `yai-cli`.
- [x] Every phase has explicit proof pointers and outcomes.
- [x] Closure review is reproducible from documented commands.
- [x] ADR-011 closure readiness is demonstrated by artifacts.
- [x] Mandatory command outcomes are recorded as `PASS` (no `SKIP` closure).

## Execution Snapshot (2026-02-21)

- Evidence bundle: `docs/20-program/24-milestone-packs/contract-baseline-lock/evidence/wave0-2026-02-21/`
- `tools/bin/yai-check-pins` -> `PASS` (`exit=0`) via `10-check_pins-strict-pass.*`
- `tools/bin/yai-docs-trace-check --all` -> `PASS` (`exit=0`) via `12-docs-trace-check-post-pin.*`
- `tools/bin/yai-proof-check` -> `PASS` (`exit=0`) via `11-proof-check-post-pin.*`
- `tools/bin/yai-proof-check --manifest docs/50-validation/proof/.private/PP-FOUNDATION-0001/pp-foundation-0001.manifest.v1.json` -> `FAIL` (`exit=2`) via `08-proof-check-private-hardfail.*`

Phase state:
- `CLOSED` (phase acceptance criteria satisfied with recorded evidence) (strict pins aligned and mandatory checks passing).
