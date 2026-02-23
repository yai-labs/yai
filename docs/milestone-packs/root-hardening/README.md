---
id: MP-ROOT-HARDENING-INDEX
status: active
runbook: docs/runbooks/root-hardening.md
phase: index
adrs:
  - docs/design/adr/ADR-012-audit-convergence-gates.md
spec_anchors:
  - deps/yai-specs/specs/protocol/include/transport.h
issues:
  - "140"
---

# Root Hardening Milestone Packs

This track is Wave 2 of audit convergence and binds Root hardening phases to claim IDs and mandatory evidence commands.

References:
- Plan: `docs/program-delivery/audit-convergence/EXECUTION-PLAN-v0.1.0.md`
- Matrix: `docs/program-delivery/audit-convergence/AUDIT-CONVERGENCE-MATRIX-v0.1.0.md`
- Claims: `docs/audits/claims/infra-grammar.v0.1.json`
- Runbook: `docs/runbooks/root-hardening.md`

Policy:
- Mandatory command outcomes are required for closure.
- `SKIP` on mandatory evidence is treated as `FAIL`.

Phase packs:
- `docs/milestone-packs/root-hardening/MP-ROOT-HARDENING-0.1.0.md`
- `docs/milestone-packs/root-hardening/MP-ROOT-HARDENING-0.1.1.md`
- `docs/milestone-packs/root-hardening/MP-ROOT-HARDENING-0.1.2.md`
- `docs/milestone-packs/root-hardening/MP-ROOT-HARDENING-0.1.3.md`
- `docs/milestone-packs/root-hardening/MP-ROOT-HARDENING-0.1.4.md`
- `docs/milestone-packs/root-hardening/MP-ROOT-HARDENING-0.1.5.md`
