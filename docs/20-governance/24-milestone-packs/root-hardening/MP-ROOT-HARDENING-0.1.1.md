---
id: MP-ROOT-HARDENING-0.1.1
status: draft
runbook: docs/20-governance/23-runbooks/root-hardening.md
phase: "0.1.1 — Root = Byte-Perfect Router"
adrs:
  - docs/20-governance/22-adr/ADR-002-root-entrypoint.md
  - docs/20-governance/22-adr/ADR-006-unified-rpc.md
  - docs/20-governance/22-adr/ADR-008-connection-lifecycle.md
  - docs/20-governance/22-adr/ADR-012-audit-convergence-gates.md
spec_anchors:
  - deps/yai-specs/specs/protocol/include/transport.h
  - deps/yai-specs/specs/protocol/include/auth.h
  - deps/yai-specs/specs/protocol/include/errors.h
claims:
  - C-ENVELOPE-HANDSHAKE-REQUIRED
  - C-DOMAIN-COVERAGE-NETWORK
evidence_commands_required:
  - tools/bin/yai-verify
  - tools/ops/suite/suite.sh
issues:
  - "140"
---

# MP-ROOT-HARDENING-0.1.1

## Metadata
- Runbook: `docs/20-governance/23-runbooks/root-hardening.md`
- Phase: `0.1.1 — Root = Byte-Perfect Router`
- Wave issue: `#140`
- Status: `draft`

## Links
- Plan: `docs/30-program/audit-convergence/EXECUTION-PLAN-v0.1.0.md`
- Matrix: `docs/30-program/audit-convergence/AUDIT-CONVERGENCE-MATRIX-v0.1.0.md`
- Claims registry: `docs/60-validation/audits/claims/infra-grammar.v0.1.json`
- ADR: `docs/20-governance/22-adr/ADR-012-audit-convergence-gates.md`

## Objective
Close phase 0.1.1 with explicit claim/evidence bindings and reproducible gate outputs.

## Mandatory command outcomes
- `tools/bin/yai-verify` -> `PASS`
- `tools/ops/suite/suite.sh` -> `PASS`

Closure policy: mandatory `SKIP` is treated as `FAIL`.

## Definition of Done
- [ ] Phase claim IDs are covered by evidence.
- [ ] Mandatory commands are recorded with exit codes and outputs.
- [ ] Root->Kernel evidence is traceable on deterministic pass/fail paths.
- [ ] MP links from runbook phase and matrix remain valid.

## Execution Snapshot
- Status: `PLANNED`
- Evidence bundle: `docs/20-governance/24-milestone-packs/root-hardening/evidence/0.1.1/`
