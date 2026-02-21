---
id: MP-CONTRACT-BASELINE-LOCK-0.1.4
status: draft
runbook: docs/runbooks/contract-baseline-lock.md
phase: "0.1.4 — Cross-Repo Evidence Closure"
adrs:
  - docs/design/adr/ADR-011-contract-baseline-lock.md
  - docs/design/adr/ADR-012-audit-convergence-gates.md
spec_anchors:
  - deps/yai-specs/contracts/invariants/I-001-traceability.md
  - deps/yai-specs/contracts/invariants/I-003-governance.md
  - deps/yai-specs/contracts/boundaries/L1-kernel.md
claims:
  - C-EVIDENCE-PACK-REPRODUCIBLE
  - C-SPEC-FIRST-PINNED
evidence_commands_required:
  - tools/release/check_pins.sh
  - tools/bin/yai-docs-trace-check --all
  - tools/bin/yai-proof-check
issues:
  - 141
---
# MP-CONTRACT-BASELINE-LOCK-0.1.4

## Metadata

- Runbook: `docs/runbooks/contract-baseline-lock.md`
- Phase: `0.1.4 — Cross-Repo Evidence Closure`
- Wave issue: `#141`
- Owner: `governance`
- Status: `draft`

## Links

- ADR: `docs/design/adr/ADR-011-contract-baseline-lock.md`
- ADR: `docs/design/adr/ADR-012-audit-convergence-gates.md`
- Proposals: `docs/design/proposals/PRP-004-contract-baseline-lock-and-pin-policy.md`, `docs/design/proposals/PRP-005-formal-coverage-roadmap.md`
- Evidence plans: `docs/test-plans/hardfail.md`
- Claims registry: `docs/audits/claims/infra-grammar.v0.1.json`

Objective:
- Close Milestone 1 with explicit, auditable cross-repo evidence for the baseline lock track.

Contract Delta:
- Envelope: none.
- Authority: none.
- Errors: closure fails if evidence set is incomplete.
- Logging: evidence index must include reproducible command/log pointers.

Repo Split:
- `yai-specs`: normative contract and governance anchors.
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
- `tools/release/check_pins.sh` -> `PASS`
- `tools/bin/yai-docs-trace-check --all` -> `PASS`
- `tools/bin/yai-proof-check` -> `PASS`

Closure policy:
- mandatory command `SKIP` is treated as `FAIL`.

Compatibility Classification:
- Type: A
- Rationale: closure/evidence hardening only; no protocol behavior change.
- Upgrade path: none required for runtime clients.

Definition of Done:
- [ ] Evidence index spans `yai-specs`, `yai`, and `yai-cli`.
- [ ] Every phase has explicit proof pointers and outcomes.
- [ ] Closure review is reproducible from documented commands.
- [ ] ADR-011 closure readiness is demonstrated by artifacts.
- [ ] Mandatory command outcomes are recorded as `PASS` (no `SKIP` closure).
