---
id: AUDITS-README
status: active
owner: governance
effective_date: 2026-02-19
revision: 2
---

# Audits

Public audits in this folder are canonical governance artifacts.

Draft or provisional audits must stay local in:
- `docs/60-validation/audits/.private/` (gitignored)

## Claims Source of Truth

Infra Grammar claims for v0.1.0 are versioned in:
- `docs/60-validation/audits/claims/infra-grammar.v0.1.json`

Program convergence references:
- `docs/20-program/audit-convergence/EXECUTION-PLAN-v0.1.0.md`
- `docs/20-program/audit-convergence/AUDIT-CONVERGENCE-MATRIX-v0.1.0.md`

## When an audit is canonical

An audit should be published in `docs/60-validation/audits/` only when all are true:

1. A milestone phase is closed (MP status aligned with delivered scope).
2. Evidence is attached and reproducible (commands, outputs, CI pointers, dates).
3. Traceability is explicit (`proposal -> ADR -> runbook -> MP -> evidence`).
4. Human maintainer review is completed.

If one of these is missing, keep the audit private and treat it as working draft.

## Suggested workflow

1. Agent prepares draft audit under `docs/60-validation/audits/.private/...`.
2. Maintainer reviews and challenges claims.
3. Promote only the finalized version to `docs/60-validation/audits/`.

## Closure Rule

For mandatory evidence checks tied to a claim or phase gate:
- `SKIP` is treated as `FAIL` for closure.
