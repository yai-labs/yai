---
id: MP-CONTRACT-BASELINE-LOCK-0.1.0
status: active
runbook: docs/20-governance/23-runbooks/contract-baseline-lock.md
phase: "0.1.0 — Pin Baseline Freeze"
adrs:
  - docs/20-governance/22-adr/ADR-011-contract-baseline-lock.md
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

- Runbook: `docs/20-governance/23-runbooks/contract-baseline-lock.md`
- Phase: `0.1.0 — Pin Baseline Freeze`
- Owner: `governance`
- Status: `active`

## Links

- ADR: `docs/20-governance/22-adr/ADR-011-contract-baseline-lock.md`
- Proposal: `docs/20-governance/21-proposals/PRP-004-contract-baseline-lock-and-pin-policy.md`
- Evidence plans: `docs/50-qualification/test-plans/contract-baseline-lock-prep.md`, `docs/50-qualification/test-plans/hardfail.md`

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
- [x] `yai` and `yai-cli` point to the same `deps/yai-specs` baseline commit.
- [x] Pin check evidence is attached and reviewable.
- [x] No contract drift is introduced by this phase.
- [x] Phase closure references ADR-011 and runbook anchor.

## Closure Evidence

Baseline commit:
51f0ef3b5985d9fbd18c8f794d03206055bc7f0d

Verification commands:
- `bash tools/release/check_pins.sh` -> PASS
- `tools/bin/yai-docs-trace-check --all` -> PASS
- `tools/bin/yai-proof-check` -> SKIP (not required for this phase)

Execution issue:
#125

Phase closure issue:
#124

PR:
#126
