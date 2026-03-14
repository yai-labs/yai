---
role: support
status: draft
audience: governance
owner_domain: program-adr
id: ADR-012
decision_id: ADR-012
supersedes: []
superseded_by: []
implements: [docs/program/rfc/rfc-005-formal-coverage-roadmap.md]
evidenced_by: [docs/program/reports/runtime-convergence-report.md]
related: []
phase: governance
runbook: docs/program/reports/audit-convergence-report.md
---
# ADR-012 - Audit Convergence Gates for v0.1.0

# Purpose
Captures architecture decision records used for governance traceability.

# Scope
Covers decision context, accepted direction, and downstream implications.

# Relationships
- Related RFCs
- Associated implementation evidence and reports

# Canonical Role
Program support artifact with decision authority in governance context.

# Main Body
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
- `ops/evidence/validation/audits/claims/infra-grammar.v0.1.json`

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
  - `docs/program/reports/audit-convergence-report.md`
- Convergence matrix:
  - `docs/program/reports/audit-convergence-report.md`
- Claims registry:
  - `ops/evidence/validation/audits/claims/infra-grammar.v0.1.json`
- Related ADR:
  - `docs/program/adr/adr-005-mind-proposer.md`
  - `docs/program/adr/adr-011-contract-runbook-lock.md`

## Status
Draft; proposed as governance baseline for v0.1.0 program delivery.

# Related Docs
- `docs/program/adr/README.md`
- Linked RFC/report artifacts
