---
id: MP-ROOT-HARDENING-0.1.2
status: draft
runbook: docs/20-program/23-runbooks/root-hardening.md
phase: "0.1.2 — Envelope-Only Authority Gate"
adrs:
  - docs/20-program/22-adr/ADR-002-root-entrypoint.md
  - docs/20-program/22-adr/ADR-006-unified-rpc.md
  - docs/20-program/22-adr/ADR-008-connection-lifecycle.md
  - docs/20-program/22-adr/ADR-012-audit-convergence-gates.md
spec_anchors:
  - deps/yai-specs/specs/protocol/include/transport.h
  - deps/yai-specs/specs/protocol/include/auth.h
  - deps/yai-specs/specs/protocol/include/errors.h
claims:
  - C-KERNEL-HARD-BOUNDARY-CORE
  - C-ENVELOPE-HANDSHAKE-REQUIRED
evidence_commands_required:
  - tools/bin/yai-verify
  - tools/bin/yai-suite
issues:
  - "140"
---

# MP-ROOT-HARDENING-0.1.2

## Metadata
- Runbook: `docs/20-program/23-runbooks/root-hardening.md`
- Phase: `0.1.2 — Envelope-Only Authority Gate`
- Wave issue: `#140`
- Status: `draft`

## Links
- Plan: `docs/20-program/audit-convergence/EXECUTION-PLAN-v0.1.0.md`
- Matrix: `docs/20-program/audit-convergence/AUDIT-CONVERGENCE-MATRIX-v0.1.0.md`
- Claims registry: `docs/50-validation/audits/claims/infra-grammar.v0.1.json`
- ADR: `docs/20-program/22-adr/ADR-012-audit-convergence-gates.md`

## Objective
Close phase 0.1.2 with explicit claim/evidence bindings and reproducible gate outputs.

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
- Status: `PLANNED`
- Evidence bundle: `docs/20-program/24-milestone-packs/root-hardening/evidence/0.1.2/`
