---
id: MP-SPECS-REFACTOR-FOUNDATION-0.1.0
status: draft
runbook: docs/20-governance/23-runbooks/specs-refactor-foundation.md
phase: "0.1.0 — Canonical Tree & Domain Separation"
adrs:
  - docs/20-governance/22-adr/ADR-011-contract-baseline-lock.md
  - docs/20-governance/22-adr/ADR-012-audit-convergence-gates.md
spec_anchors:
  - deps/yai-specs/SPEC_MAP.md
  - deps/yai-specs/REGISTRY.md
claims:
  - C-EVIDENCE-PACK-REPRODUCIBLE
evidence_commands_required:
  - tools/bin/yai-docs-trace-check --all
issues:
  - "142"
  - "yai-specs#9"
---

# MP-SPECS-REFACTOR-FOUNDATION-0.1.0

## Metadata
- Runbook: `docs/20-governance/23-runbooks/specs-refactor-foundation.md`
- Phase: `0.1.0 — Canonical Tree & Domain Separation`
- Wave issue: `#142`
- Specs branch issue: `yai-specs#9`
- Status: `draft`

## Links
- Plan: `docs/30-program/audit-convergence/EXECUTION-PLAN-v0.1.0.md`
- Matrix: `docs/30-program/audit-convergence/AUDIT-CONVERGENCE-MATRIX-v0.1.0.md`
- Claims registry: `docs/60-validation/audits/claims/infra-grammar.v0.1.json`
- ADR: `docs/20-governance/22-adr/ADR-011-contract-baseline-lock.md`
- ADR: `docs/20-governance/22-adr/ADR-012-audit-convergence-gates.md`

## Objective
Close phase 0.1.0 with explicit claim/evidence bindings and reproducible gate outputs.

## Mandatory command outcomes
- `tools/bin/yai-docs-trace-check --all` -> `PASS`

Closure policy: mandatory `SKIP` is treated as `FAIL`.

## Definition of Done
- [ ] Phase claim IDs are covered by evidence.
- [ ] Mandatory commands are recorded with exit codes and outputs.
- [ ] Cross-repo references are traceable (yai <-> yai-specs <-> yai-cli).
- [ ] MP links from runbook phase and matrix remain valid.

## Execution Snapshot
- Status: `PLANNED`
- Evidence bundle: `docs/20-governance/24-milestone-packs/specs-refactor-foundation/evidence/0.1.0/`
