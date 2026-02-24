---
id: PROGRAM-DELIVERY-AUDIT-CONVERGENCE
status: active
owner: governance
updated: 2026-02-21
related:
  - docs/20-program/audit-convergence/EXECUTION-PLAN-v0.1.0.md
  - docs/20-program/audit-convergence/AUDIT-CONVERGENCE-MATRIX-v0.1.0.md
  - docs/50-validation/audits/claims/infra-grammar.v0.1.json
  - docs/20-program/22-adr/ADR-012-audit-convergence-gates.md
issue:
  - https://github.com/yai-labs/yai/issues/140
---

# Audit Convergence (v0.1.0)

This folder is the canonical backbone for converging runbooks/ADR/MP to one target:
Infra Grammar audit green on all domains, including Mind.

Canonical artifacts:
- Execution plan: `docs/20-program/audit-convergence/EXECUTION-PLAN-v0.1.0.md`
- Convergence matrix: `docs/20-program/audit-convergence/AUDIT-CONVERGENCE-MATRIX-v0.1.0.md`
- Claims registry: `docs/50-validation/audits/claims/infra-grammar.v0.1.json`
- Governance ADR: `docs/20-program/22-adr/ADR-012-audit-convergence-gates.md`

Rules:
- Source of truth for claims is the registry JSON.
- SKIP on mandatory evidence checks is FAIL.
- Gate A (Core) and Gate B (Mind) are distinct closure checkpoints.
- In-flight runbook execution is not rewritten mid-phase; re-centering happens at phase boundaries.
- Consumer `deps/` trees are read-only for this program; normative changes are made only in `yai-law` branches.
