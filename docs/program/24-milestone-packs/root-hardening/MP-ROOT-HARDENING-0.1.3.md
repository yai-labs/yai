---
id: MP-ROOT-HARDENING-0.1.3
status: active
runbook: docs/program/23-runbooks/root-hardening.md
phase: "0.1.3 — ws_id Validation Centralization"
adrs:
  - docs/program/22-adr/ADR-002-root-entrypoint.md
  - docs/program/22-adr/ADR-006-unified-rpc.md
  - docs/program/22-adr/ADR-008-connection-lifecycle.md
  - docs/program/22-adr/ADR-012-audit-convergence-gates.md
spec_anchors:
  - deps/law/contracts/protocol/include/transport.h
  - deps/law/contracts/protocol/include/auth.h
  - deps/law/contracts/protocol/include/errors.h
claims:
  - C-CONTEXT-PROPAGATION
  - C-KERNEL-HARD-BOUNDARY-CORE
evidence_commands_required:
  - tools/bin/yai-verify
  - tools/bin/yai-suite
issues:
  - "191"
---

# MP-ROOT-HARDENING-0.1.3

## Metadata
- Runbook: `docs/program/23-runbooks/root-hardening.md`
- Phase: `0.1.3 — ws_id Validation Centralization`
- Wave issue: `#191`
- Status: `active`

## Links
- Plan: `docs/program/audit-convergence/EXECUTION-PLAN-v0.1.0.md`
- Matrix: `docs/program/audit-convergence/AUDIT-CONVERGENCE-MATRIX-v0.1.0.md`
- Claims registry: `ops/evidence/validation/audits/claims/infra-grammar.v0.1.json`
- ADR: `docs/program/22-adr/ADR-012-audit-convergence-gates.md`

## Objective
Close phase 0.1.3 with explicit claim/evidence bindings and reproducible gate outputs.

## Mandatory command outcomes
- `tools/bin/yai-verify` -> `PASS`
- `tools/bin/yai-suite` -> `PASS`

Closure policy: mandatory `SKIP` is treated as `FAIL`.

## Definition of Done
- [ ] Phase claim IDs are covered by evidence.
- [ ] Mandatory commands are recorded with exit codes and outputs.
- [ ] Root->Kernel evidence is traceable on deterministic pass/fail paths.
- [ ] MP links from runbook phase and matrix remain valid.

## Execution Snapshot
- Status: `IN_PROGRESS`
- Evidence bundle: `docs/program/24-milestone-packs/root-hardening/evidence/0.1.3/`
- Commands:
  - `tools/bin/yai-verify core` -> `PASS`
  - `tools/bin/yai-verify law-kernel` -> `PASS`
  - `tools/bin/yai-check-pins` -> `PASS`
- Notes: runtime-side ws_id validation is centralized in `root/include/ws_id.h`; CLI-side validator alignment remains open in this phase.
