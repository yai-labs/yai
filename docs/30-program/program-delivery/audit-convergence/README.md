---
id: PROGRAM-DELIVERY-AUDIT-CONVERGENCE
status: active
owner: governance
updated: 2026-02-21
related:
  - docs/program-delivery/audit-convergence/EXECUTION-PLAN-v0.1.0.md
  - docs/program-delivery/audit-convergence/AUDIT-CONVERGENCE-MATRIX-v0.1.0.md
  - docs/audits/claims/infra-grammar.v0.1.json
  - docs/design/adr/ADR-012-audit-convergence-gates.md
issue:
  - https://github.com/yai-labs/yai/issues/140
---

# Audit Convergence (v0.1.0)

This folder is the canonical backbone for converging runbooks/ADR/MP to one target:
Infra Grammar audit green on all domains, including Mind.

Canonical artifacts:
- Execution plan: `docs/program-delivery/audit-convergence/EXECUTION-PLAN-v0.1.0.md`
- Convergence matrix: `docs/program-delivery/audit-convergence/AUDIT-CONVERGENCE-MATRIX-v0.1.0.md`
- Claims registry: `docs/audits/claims/infra-grammar.v0.1.json`
- Governance ADR: `docs/design/adr/ADR-012-audit-convergence-gates.md`

Rules:
- Source of truth for claims is the registry JSON.
- SKIP on mandatory evidence checks is FAIL.
- Gate A (Core) and Gate B (Mind) are distinct closure checkpoints.
- In-flight runbook execution is not rewritten mid-phase; re-centering happens at phase boundaries.
- Consumer `deps/` trees are read-only for this program; normative changes are made only in `yai-specs` branches.
