---
id: MP-SPECS-REFACTOR-FOUNDATION-0.1.8
status: draft
runbook: docs/program/milestone-packs/runtime-baselines/specs-refactor-foundation.md
phase: "0.1.8 — TLA Reboot & Model-Check CI"
adrs:
  - docs/program/adr/ADR-011-contract-baseline-lock.md
  - docs/program/adr/ADR-012-audit-convergence-gates.md
spec_anchors:
  - ../law/SPEC_MAP.md
  - ../law/REGISTRY.md
claims:
  - C-AUTHORITY-SURFACE-RUNTIME
  - C-EVIDENCE-PACK-REPRODUCIBLE
evidence_commands_required:
  - tools/bin/yai-proof-check
  - tools/bin/yai-docs-trace-check --all
issues:
  - "142"
  - "law#9"
---

# MP-SPECS-REFACTOR-FOUNDATION-0.1.8

## Metadata
- Runbook: `docs/program/milestone-packs/runtime-baselines/specs-refactor-foundation.md`
- Phase: `0.1.8 — TLA Reboot & Model-Check CI`
- Wave issue: `#142`
- Specs branch issue: `law#9`
- Status: `draft`

## Links
- Plan: `docs/program/reports/audit-convergence/EXECUTION-PLAN-v0.1.0.md`
- Matrix: `docs/program/reports/audit-convergence/AUDIT-CONVERGENCE-MATRIX-v0.1.0.md`
- Claims registry: `ops/evidence/validation/audits/claims/infra-grammar.v0.1.json`
- ADR: `docs/program/adr/ADR-011-contract-baseline-lock.md`
- ADR: `docs/program/adr/ADR-012-audit-convergence-gates.md`

## Objective
Close phase 0.1.8 with explicit claim/evidence bindings and reproducible gate outputs.

## Mandatory command outcomes
- `tools/bin/yai-proof-check` -> `PASS`
- `tools/bin/yai-docs-trace-check --all` -> `PASS`

Closure policy: mandatory `SKIP` is treated as `FAIL`.

## Definition of Done
- [ ] Phase claim IDs are covered by evidence.
- [ ] Mandatory commands are recorded with exit codes and outputs.
- [ ] Cross-repo references are traceable (yai <-> law <-> cli).
- [ ] MP links from runbook phase and matrix remain valid.

## Execution Snapshot
- Status: `PLANNED`
- Evidence bundle: `docs/program/milestone-packs/specs-refactor-foundation/evidence/0.1.8/`
