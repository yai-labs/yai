---
id: MP-SPECS-REFACTOR-FOUNDATION-0.1.0
status: active
runbook: docs/20-program/23-runbooks/specs-refactor-foundation.md
phase: "0.1.0 — Canonical Tree & Domain Separation"
adrs:
  - docs/20-program/22-adr/ADR-011-contract-baseline-lock.md
  - docs/20-program/22-adr/ADR-012-audit-convergence-gates.md
spec_anchors:
  - deps/yai-law/SPEC_MAP.md
  - deps/yai-law/REGISTRY.md
claims:
  - C-EVIDENCE-PACK-REPRODUCIBLE
evidence_commands_required:
  - tools/bin/yai-docs-trace-check --all
issues:
  - "142"
  - "yai-law#9"
---

# MP-SPECS-REFACTOR-FOUNDATION-0.1.0

## Metadata
- Runbook: `docs/20-program/23-runbooks/specs-refactor-foundation.md`
- Phase: `0.1.0 — Canonical Tree & Domain Separation`
- Wave issue: `#142`
- Specs branch issue: `yai-law#9`
- Status: `active`

## Links
- Plan: `docs/20-program/audit-convergence/EXECUTION-PLAN-v0.1.0.md`
- Matrix: `docs/20-program/audit-convergence/AUDIT-CONVERGENCE-MATRIX-v0.1.0.md`
- Claims registry: `docs/50-validation/audits/claims/infra-grammar.v0.1.json`
- ADR: `docs/20-program/22-adr/ADR-011-contract-baseline-lock.md`
- ADR: `docs/20-program/22-adr/ADR-012-audit-convergence-gates.md`

## Objective
Close phase 0.1.0 with explicit claim/evidence bindings and reproducible gate outputs.

## Mandatory command outcomes
- `tools/bin/yai-docs-trace-check --all` -> `PASS`

Closure policy: mandatory `SKIP` is treated as `FAIL`.

## Definition of Done
- [x] Phase claim IDs are covered by evidence.
- [x] Mandatory commands are recorded with exit codes and outputs.
- [x] Cross-repo references are traceable (yai <-> yai-law <-> yai-cli).
- [x] MP links from runbook phase and matrix remain valid.

## Execution Snapshot
- Status: `CLOSED`
- Evidence bundle: `docs/20-program/24-milestone-packs/specs-refactor-foundation/evidence/0.1.0/`


Phase state:
- `CLOSED` (phase acceptance criteria satisfied for Wave 1 scope).
