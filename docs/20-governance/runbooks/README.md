---
id: RB-RUNBOOKS-README
status: active
adr_refs:
  - docs/20-governance/design/adr/ADR-011-contract-baseline-lock.md
  - docs/20-governance/design/adr/ADR-012-audit-convergence-gates.md
decisions:
  - docs/20-governance/design/adr/ADR-011-contract-baseline-lock.md
  - docs/20-governance/design/adr/ADR-012-audit-convergence-gates.md
---

# Runbooks

Runbooks translate decisions into phased, repeatable execution.

A good runbook must be deterministic:
- clear preconditions,
- explicit procedure,
- verifiable outcomes,
- rollback/failure handling.

## Typical runbooks in this repo

- `docs/20-governance/runbooks/contract-baseline-lock.md`
- `docs/20-governance/runbooks/specs-refactor-foundation.md`
- `docs/20-governance/runbooks/root-hardening.md`
- `docs/20-governance/runbooks/workspaces-lifecycle.md`
- `docs/20-governance/runbooks/engine-attach.md`
- `docs/20-governance/runbooks/data-plane.md`
- `docs/20-governance/runbooks/mind-redis-stm.md`
- `docs/20-governance/runbooks/kernel-sovereignty.md`
- `docs/20-governance/runbooks/operations.md`

## Program Convergence Backbone

Program-level target-state and wave ordering are defined in:
- `docs/30-program/program-delivery/audit-convergence/EXECUTION-PLAN-v0.1.0.md`
- `docs/30-program/program-delivery/audit-convergence/AUDIT-CONVERGENCE-MATRIX-v0.1.0.md`

Runbook phase closure should reference claim IDs from:
- `docs/60-validation/audits/claims/infra-grammar.v0.1.json`

## Template

- `docs/20-governance/templates/runbooks/RB-000-template.md`

## Traceability expectation

Runbooks should link:
- upstream ADRs and law/spec anchors,
- downstream milestone packs as phases are delivered.
