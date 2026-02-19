---
id: RB-CONTRACT-BASELINE-LOCK
title: Contract Baseline Lock
status: draft
owner: governance
effective_date: 2026-02-19
revision: 1
supersedes: []
depends_on: []
adr_refs:
  - docs/design/adr/ADR-011-contract-baseline-lock.md
decisions:
  - docs/design/adr/ADR-011-contract-baseline-lock.md
related:
  adr:
    - docs/design/adr/ADR-011-contract-baseline-lock.md
  specs:
    - deps/yai-specs/contracts/invariants/I-001-traceability.md
    - deps/yai-specs/contracts/invariants/I-002-determinism.md
    - deps/yai-specs/contracts/invariants/I-003-governance.md
    - deps/yai-specs/contracts/invariants/I-006-external-effect-boundary.md
    - deps/yai-specs/contracts/boundaries/L1-kernel.md
  test_plans:
    - docs/test-plans/hardfail.md
  tools:
    - tools/release/check_pins.sh
    - tools/bin/yai-docs-trace-check
    - tools/bin/yai-proof-check
tags:
  - governance
  - baseline-lock
---

# RB-CONTRACT-BASELINE-LOCK — Contract Baseline Lock (Milestone 1)

## 1) Purpose
Create the first governance runbook that locks cross-repo contract behavior across `yai-specs`, `yai`, and `yai-cli` before any additional hardening tracks.

## 2) Preconditions
- [ ] `docs/design/adr/ADR-011-contract-baseline-lock.md` is present and traceable.
- [ ] Cross-repo repositories are reachable (`yai-specs`, `yai`, `yai-cli`).
- [ ] Contract edits are performed in source repo (`yai-specs`) and only pinned in consumers.
- [ ] Baseline CI is green in all three repos before phase work starts.

## 3) Inputs
- Pin references:
  - `yai/deps/yai-specs`
  - `yai-cli/deps/yai-specs`
- Normative anchors:
  - `deps/yai-specs/contracts/invariants/I-001-traceability.md`
  - `deps/yai-specs/contracts/invariants/I-002-determinism.md`
  - `deps/yai-specs/contracts/invariants/I-003-governance.md`
  - `deps/yai-specs/contracts/invariants/I-006-external-effect-boundary.md`
- Verification tooling:
  - `tools/release/check_pins.sh`
  - `tools/bin/yai-docs-trace-check`
  - `tools/bin/yai-proof-check`

## 4) Procedure

### Position in the global sequence
1. Contract baseline lock (this runbook; first governance runbook).
2. Specs refactor foundation (`docs/runbooks/specs-refactor-foundation.md`).
3. Root hardening (`docs/runbooks/root-hardening.md`).
4. Workspace lifecycle / engine attach / data-plane tracks.

## 5) Phases

<a id="phase-0-1-0-pin-baseline-freeze"></a>
### 0.1.0 — Pin Baseline Freeze
Claim: `yai` and `yai-cli` consume the same audited `yai-specs` baseline.
- Scope: pin update + lock verification only.
- Gate: pin checks green and reproducible.
- Milestone Pack (planned): `docs/milestone-packs/contract-baseline-lock/MP-CONTRACT-BASELINE-LOCK-0.1.0.md`

<a id="phase-0-1-1-ci-parity"></a>
### 0.1.1 — CI Parity on Contract Surfaces
Claim: CI validates contract parity in both consumers with same baseline.
- Scope: CI parity checks for protocol/authority surfaces.
- Gate: parity checks fail on drift.
- Milestone Pack (planned): `docs/milestone-packs/contract-baseline-lock/MP-CONTRACT-BASELINE-LOCK-0.1.1.md`

<a id="phase-0-1-2-no-pass-on-skip"></a>
### 0.1.2 — No Pass-on-Skip Enforcement
Claim: required checks cannot pass via skip placeholders.
- Scope: mandatory gate behavior for proof-relevant checks.
- Gate: pipeline fails if mandatory evidence is skipped.
- Milestone Pack (planned): `docs/milestone-packs/contract-baseline-lock/MP-CONTRACT-BASELINE-LOCK-0.1.2.md`

<a id="phase-0-1-3-formal-core-sync"></a>
### 0.1.3 — Formal/Core Sync on Contract Delta
Claim: authority/envelope contract deltas trigger required formal/core verification updates.
- Scope: binding between contract delta and verification obligations.
- Gate: deltas blocked without required verify updates.
- Milestone Pack (planned): `docs/milestone-packs/contract-baseline-lock/MP-CONTRACT-BASELINE-LOCK-0.1.3.md`

<a id="phase-0-1-4-cross-repo-evidence"></a>
### 0.1.4 — Cross-Repo Evidence Closure
Claim: closure evidence is explicit and auditable across all repos.
- Scope: evidence index and deterministic positive/negative checks.
- Gate: MP evidence complete and reviewable.
- Milestone Pack (planned): `docs/milestone-packs/contract-baseline-lock/MP-CONTRACT-BASELINE-LOCK-0.1.4.md`

## 6) Verification
- `tools/release/check_pins.sh`
- `tools/bin/yai-docs-trace-check --all`
- `tools/bin/yai-proof-check --all`

## 7) Failure Modes
- Symptom: `yai` and `yai-cli` pin different specs commits.
  - Fix: resync pins and rerun parity checks.
- Symptom: CI green despite missing mandatory proof checks.
  - Fix: convert skip path to hard fail and rerun.
- Symptom: contract delta merged without formal/core sync.
  - Fix: block merge until required verification deltas are present.

## 8) Rollback
- Roll back consumer pin refs to previous audited baseline.
- Revert lock-policy changes for the current phase only.
- Re-run pin + parity checks before reopening PR.

## 9) References
- ADR: `docs/design/adr/ADR-011-contract-baseline-lock.md`
- Next runbook: `docs/runbooks/specs-refactor-foundation.md`
- MP track: `docs/milestone-packs/contract-baseline-lock/README.md`

## Traceability

- ADR refs (required unless ops-only):
  - `docs/design/adr/ADR-011-contract-baseline-lock.md`
- Law refs (recommended):
  - `deps/yai-specs/contracts/invariants/I-001-traceability.md`
  - `deps/yai-specs/contracts/invariants/I-002-determinism.md`
  - `deps/yai-specs/contracts/invariants/I-003-governance.md`
  - `deps/yai-specs/contracts/invariants/I-006-external-effect-boundary.md`
- MPs (filled as phases ship):
  - `docs/milestone-packs/contract-baseline-lock/MP-CONTRACT-BASELINE-LOCK-0.1.0.md`
  - `docs/milestone-packs/contract-baseline-lock/MP-CONTRACT-BASELINE-LOCK-0.1.1.md`
  - `docs/milestone-packs/contract-baseline-lock/MP-CONTRACT-BASELINE-LOCK-0.1.2.md`
  - `docs/milestone-packs/contract-baseline-lock/MP-CONTRACT-BASELINE-LOCK-0.1.3.md`
  - `docs/milestone-packs/contract-baseline-lock/MP-CONTRACT-BASELINE-LOCK-0.1.4.md`
