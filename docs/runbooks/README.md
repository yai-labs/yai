---
id: RB-RUNBOOKS-README
status: active
adr_refs:
  - docs/design/adr/ADR-011-contract-baseline-lock.md
  - docs/design/adr/ADR-012-audit-convergence-gates.md
decisions:
  - docs/design/adr/ADR-011-contract-baseline-lock.md
  - docs/design/adr/ADR-012-audit-convergence-gates.md
---

# Runbooks

Runbooks translate decisions into phased, repeatable execution.

A good runbook must be deterministic:
- clear preconditions,
- explicit procedure,
- verifiable outcomes,
- rollback/failure handling.

## Typical runbooks in this repo

- `docs/runbooks/contract-baseline-lock.md`
- `docs/runbooks/specs-refactor-foundation.md`
- `docs/runbooks/root-hardening.md`
- `docs/runbooks/workspaces-lifecycle.md`
- `docs/runbooks/engine-attach.md`
- `docs/runbooks/data-plane.md`
- `docs/runbooks/mind-redis-stm.md`
- `docs/runbooks/kernel-sovereignty.md`
- `docs/runbooks/operations.md`

## Program Convergence Backbone

Program-level target-state and wave ordering are defined in:
- `docs/program-delivery/audit-convergence/EXECUTION-PLAN-v0.1.0.md`
- `docs/program-delivery/audit-convergence/AUDIT-CONVERGENCE-MATRIX-v0.1.0.md`

Runbook phase closure should reference claim IDs from:
- `docs/audits/claims/infra-grammar.v0.1.json`

## Template

- `docs/templates/runbooks/RB-000-template.md`

## Traceability expectation

Runbooks should link:
- upstream ADRs and law/spec anchors,
- downstream milestone packs as phases are delivered.
