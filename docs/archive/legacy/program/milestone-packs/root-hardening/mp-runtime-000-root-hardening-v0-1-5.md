---
id: MP-ROOT-HARDENING-0.1.5
status: draft
runbook: docs/archive/legacy/program/milestone-packs/runtime-baselines/operations-foundation/mp-runtime-000-root-hardening.md
phase: "0.1.5 — Test Matrix + Torture Suite"
adrs:
  - docs/program/adr/adr-runtime-002-root-entrypoint.md
  - docs/program/adr/adr-protocol-006-unified-rpc.md
  - docs/program/adr/adr-workspace-008-connection-lifecycle.md
  - docs/program/adr/adr-program-012-audit-convergence-gates.md
spec_anchors:
  - ../governance/contracts/protocol/include/transport.h
  - ../governance/contracts/protocol/include/auth.h
  - ../governance/contracts/protocol/include/errors.h
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
- Runbook: `docs/archive/legacy/program/milestone-packs/runtime-baselines/operations-foundation/mp-runtime-000-root-hardening.md`
- Phase: `0.1.5 — Test Matrix + Torture Suite`
- Wave issue: `#191`
- Status: `draft`

## Links
- Plan: `docs/program/reports/audit-convergence-report.md`
- Matrix: `docs/program/reports/audit-convergence-report.md`
- Claims registry: `ops/evidence/validation/audits/claims/infra-grammar.v0.1.json`
- ADR: `docs/program/adr/adr-program-012-audit-convergence-gates.md`

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
