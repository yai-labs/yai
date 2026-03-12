---
id: MP-SPECS-REFACTOR-FOUNDATION-0.1.8
status: draft
runbook: docs/archive/legacy/program/milestone-packs/runtime-baselines/operations-foundation/mp-runtime-000-specs-refactor-foundation.md
phase: "0.1.8 — TLA Reboot & Model-Check CI"
adrs:
  - docs/program/adr/adr-contracts-011-contract-runbook-lock.md
  - docs/program/adr/adr-program-012-audit-convergence-gates.md
spec_anchors:
  - ../governance/SPEC_MAP.md
  - ../governance/REGISTRY.md
claims:
  - C-AUTHORITY-SURFACE-RUNTIME
  - C-EVIDENCE-PACK-REPRODUCIBLE
evidence_commands_required:
  - tools/bin/yai-proof-check
  - tools/bin/yai-docs-trace-check --all
issues:
  - "142"
  - "governance#9"
---

# MP-SPECS-REFACTOR-FOUNDATION-0.1.8

## Metadata
- Runbook: `docs/archive/legacy/program/milestone-packs/runtime-baselines/operations-foundation/mp-runtime-000-specs-refactor-foundation.md`
- Phase: `0.1.8 — TLA Reboot & Model-Check CI`
- Wave issue: `#142`
- Specs branch issue: `governance#9`
- Status: `draft`

## Links
- Plan: `docs/program/reports/audit-convergence-report.md`
- Matrix: `docs/program/reports/audit-convergence-report.md`
- Claims registry: `ops/evidence/validation/audits/claims/infra-grammar.v0.1.json`
- ADR: `docs/program/adr/adr-contracts-011-contract-runbook-lock.md`
- ADR: `docs/program/adr/adr-program-012-audit-convergence-gates.md`

## Objective
Close phase 0.1.8 with explicit claim/evidence bindings and reproducible gate outputs.

## Mandatory command outcomes
- `tools/bin/yai-proof-check` -> `PASS`
- `tools/bin/yai-docs-trace-check --all` -> `PASS`

Closure policy: mandatory `SKIP` is treated as `FAIL`.

## Definition of Done
- [ ] Phase claim IDs are covered by evidence.
- [ ] Mandatory commands are recorded with exit codes and outputs.
- [ ] Cross-repo references are traceable (yai <-> governance <-> cli).
- [ ] MP links from runbook phase and matrix remain valid.

## Execution Snapshot
- Status: `PLANNED`
- Evidence bundle: `docs/program/milestone-packs/specs-refactor-foundation/evidence/0.1.8/`
