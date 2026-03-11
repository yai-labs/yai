---
id: MP-ROOT-HARDENING-0.1.5
status: draft
runbook: docs/program/milestone-packs/runtime-baselines/root-hardening.md
phase: "0.1.5 — Test Matrix + Torture Suite"
adrs:
  - docs/program/adr/ADR-002-root-entrypoint.md
  - docs/program/adr/ADR-006-unified-rpc.md
  - docs/program/adr/ADR-008-connection-lifecycle.md
  - docs/program/adr/ADR-012-audit-convergence-gates.md
spec_anchors:
  - ../law/contracts/protocol/include/transport.h
  - ../law/contracts/protocol/include/auth.h
  - ../law/contracts/protocol/include/errors.h
claims:
  - C-DOMAIN-COVERAGE-NETWORK
  - C-KERNEL-HARD-BOUNDARY-CORE
  - C-EVIDENCE-PACK-REPRODUCIBLE
evidence_commands_required:
  - tools/bin/yai-verify
  - tools/bin/yai-suite
  - tools/bin/yai-proof-check
issues:
  - "191"
---

# MP-ROOT-HARDENING-0.1.5

## Metadata
- Runbook: `docs/program/milestone-packs/runtime-baselines/root-hardening.md`
- Phase: `0.1.5 — Test Matrix + Torture Suite`
- Wave issue: `#191`
- Status: `draft`

## Links
- Plan: `docs/program/reports/audit-convergence/EXECUTION-PLAN-v0.1.0.md`
- Matrix: `docs/program/reports/audit-convergence/AUDIT-CONVERGENCE-MATRIX-v0.1.0.md`
- Claims registry: `ops/evidence/validation/audits/claims/infra-grammar.v0.1.json`
- ADR: `docs/program/adr/ADR-012-audit-convergence-gates.md`

## Objective
Close phase 0.1.5 with explicit claim/evidence bindings and reproducible gate outputs.

## Mandatory command outcomes
- `tools/bin/yai-verify` -> `PASS`
- `tools/bin/yai-suite` -> `PASS`
- `tools/bin/yai-proof-check` -> `PASS`

Closure policy: mandatory `SKIP` is treated as `FAIL`.

## Definition of Done
- [ ] Phase claim IDs are covered by evidence.
- [ ] Mandatory commands are recorded with exit codes and outputs.
- [ ] Root->Kernel evidence is traceable on deterministic pass/fail paths.
- [ ] MP links from runbook phase and matrix remain valid.

## Execution Snapshot
- Status: `PLANNED`
- Evidence bundle: `docs/program/milestone-packs/root-hardening/evidence/0.1.5/`
