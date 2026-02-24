---
id: ADR-012
status: draft
effective_date: 2026-02-21
supersedes: []
applies_to:
  runbook: docs/20-program/audit-convergence/EXECUTION-PLAN-v0.1.0.md
  phase: governance
  anchor: "#4-two-official-gates"
law_refs:
  - deps/yai-law/contracts/invariants/I-001-traceability.md
  - deps/yai-law/contracts/invariants/I-002-determinism.md
  - deps/yai-law/contracts/invariants/I-003-governance.md
  - deps/yai-law/contracts/invariants/I-006-external-effect-boundary.md
  - deps/yai-law/contracts/boundaries/L1-kernel.md
---

# ADR-012 - Audit Convergence Gates for v0.1.0

## Context
The Infra Grammar audit exposed a program-level coordination gap:
runbooks and milestone packs were valuable but not converging through one explicit closure model.

Two risks were recurrent:
- scope drift across parallel tracks,
- "green" claims without explicit enforcement level and without consistent evidence closure semantics.

## Decision
Adopt a two-gate convergence model for v0.1.0:

1. Gate A (`Audit Green Core`): all core domains (control/network/providers/storage/resource/audit) satisfy grammar closure.
2. Gate B (`Audit Green Mind`): Mind proposer-only integration satisfies grammar closure on top of Gate A.

Define v0.1.0 GREEN explicitly as:
- hard runtime mediation at Root/Kernel/Engine boundaries,
- not full OS-hardening completeness.

Enforce policy:
- mandatory-check `SKIP` is treated as `FAIL` for phase closure.

Claims are canonicalized in:
- `docs/50-validation/audits/claims/infra-grammar.v0.1.json`

## Rationale
The two-gate model keeps one trajectory while avoiding false blocking:
- core can close first with real enforcement/evidence quality,
- mind can close second without diluting authority boundaries.

The explicit GREEN definition prevents over-claiming and keeps delivery aligned to current TRL.

## Consequences
- Positive:
  - one deterministic convergence model across runbooks and MPs,
  - lower risk of synthetic progress,
  - explicit anti-skip governance.
- Negative:
  - additional documentation maintenance (registry + matrix alignment),
  - stricter closure requirements can reduce short-term velocity.

## Traceability
- Execution plan:
  - `docs/20-program/audit-convergence/EXECUTION-PLAN-v0.1.0.md`
- Convergence matrix:
  - `docs/20-program/audit-convergence/AUDIT-CONVERGENCE-MATRIX-v0.1.0.md`
- Claims registry:
  - `docs/50-validation/audits/claims/infra-grammar.v0.1.json`
- Related ADR:
  - `docs/20-program/22-adr/ADR-005-mind-proposer.md`
  - `docs/20-program/22-adr/ADR-011-contract-baseline-lock.md`

## Status
Draft; proposed as governance baseline for v0.1.0 program delivery.
