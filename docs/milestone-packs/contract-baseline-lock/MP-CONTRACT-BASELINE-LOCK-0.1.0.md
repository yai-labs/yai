---
id: MP-CONTRACT-BASELINE-LOCK-0.1.0
status: draft
runbook: docs/runbooks/contract-baseline-lock.md
phase: "0.1.0 — Pin Baseline Freeze"
adrs:
  - docs/design/adr/ADR-011-contract-baseline-lock.md
spec_anchors:
  - deps/yai-specs/contracts/invariants/I-001-traceability.md
  - deps/yai-specs/contracts/invariants/I-002-determinism.md
  - deps/yai-specs/contracts/invariants/I-003-governance.md
  - deps/yai-specs/contracts/invariants/I-006-external-effect-boundary.md
issues:
  - "https://github.com/yai-labs/yai/issues/125"
issue_reason: "Phase tracked by governance runbook issue #125."
---
# MP-CONTRACT-BASELINE-LOCK-0.1.0

## Metadata

- Runbook: `docs/runbooks/contract-baseline-lock.md`
- Phase: `0.1.0 — Pin Baseline Freeze`
- Owner: `governance`
- Status: `draft`

## Links

- ADR: `docs/design/adr/ADR-011-contract-baseline-lock.md`
- Proposal: `docs/design/proposals/PRP-004-contract-baseline-lock-and-pin-policy.md`
- Evidence plans: `docs/test-plans/contract-baseline-lock-prep.md`, `docs/test-plans/hardfail.md`

Objective:
- Ensure `yai` and `yai-cli` consume the same audited `yai-specs` baseline commit.

Contract Delta:
- Envelope: none.
- Authority: none.
- Errors: none.
- Logging: pin-check execution evidence must be recorded.

Repo Split:
- `yai`: align `deps/yai-specs` ref to audited baseline.
- `yai-cli`: align `deps/yai-specs` ref to the same audited baseline.

Evidence Plan (minimum):
- Positive cases:
  - `tools/release/check_pins.sh` confirms aligned pins.
  - Consumer verify commands pass with the aligned baseline.
- Negative cases:
  - Mismatched pin between repos is detected by pin check.
  - Non-audited pin update is rejected by review gate.

Compatibility Classification:
- Type: A
- Rationale: no contract semantics change; only baseline alignment.
- Upgrade path: existing conformant clients remain valid.

Definition of Done:
- [ ] `yai` and `yai-cli` point to the same `deps/yai-specs` baseline commit.
- [ ] Pin check evidence is attached and reviewable.
- [ ] No contract drift is introduced by this phase.
- [ ] Phase closure references ADR-011 and runbook anchor.
