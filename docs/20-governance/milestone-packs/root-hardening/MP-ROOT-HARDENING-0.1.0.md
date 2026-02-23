---
id: MP-ROOT-HARDENING-0.1.0
status: draft
runbook: docs/runbooks/root-hardening.md
phase: "0.1.0 — Protocol Guardrails"
adrs:
  - docs/design/adr/ADR-002-root-entrypoint.md
  - docs/design/adr/ADR-006-unified-rpc.md
  - docs/design/adr/ADR-008-connection-lifecycle.md
  - docs/design/adr/ADR-012-audit-convergence-gates.md
spec_anchors:
  - deps/yai-specs/specs/protocol/include/transport.h
  - deps/yai-specs/specs/protocol/include/auth.h
  - deps/yai-specs/specs/protocol/include/errors.h
claims:
  - C-ENVELOPE-HANDSHAKE-REQUIRED
  - C-DOMAIN-COVERAGE-NETWORK
evidence_commands_required:
  - tools/bin/yai-verify
issues:
  - "140"
---

# MP-ROOT-HARDENING-0.1.0

## Metadata
- Runbook: `docs/runbooks/root-hardening.md`
- Phase: `0.1.0 — Protocol Guardrails`
- Wave issue: `#140`
- Status: `draft`

## Links
- Plan: `docs/program-delivery/audit-convergence/EXECUTION-PLAN-v0.1.0.md`
- Matrix: `docs/program-delivery/audit-convergence/AUDIT-CONVERGENCE-MATRIX-v0.1.0.md`
- Claims registry: `docs/audits/claims/infra-grammar.v0.1.json`
- ADR: `docs/design/adr/ADR-012-audit-convergence-gates.md`

## Objective
Close phase 0.1.0 with explicit claim/evidence bindings and reproducible gate outputs.

## Mandatory command outcomes
- `tools/bin/yai-verify` -> `PASS`

Closure policy: mandatory `SKIP` is treated as `FAIL`.

## Definition of Done
- [ ] Phase claim IDs are covered by evidence.
- [ ] Mandatory commands are recorded with exit codes and outputs.
- [ ] Root->Kernel evidence is traceable on deterministic pass/fail paths.
- [ ] MP links from runbook phase and matrix remain valid.

## Execution Snapshot
- Status: `PLANNED`
- Evidence bundle: `docs/milestone-packs/root-hardening/evidence/0.1.0/`
