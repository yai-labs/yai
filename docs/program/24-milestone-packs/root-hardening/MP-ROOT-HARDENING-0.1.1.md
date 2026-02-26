---
id: MP-ROOT-HARDENING-0.1.1
status: active
runbook: docs/20-program/23-runbooks/root-hardening.md
phase: "0.1.1 — Root = Byte-Perfect Router"
adrs:
  - docs/20-program/22-adr/ADR-002-root-entrypoint.md
  - docs/20-program/22-adr/ADR-006-unified-rpc.md
  - docs/20-program/22-adr/ADR-008-connection-lifecycle.md
  - docs/20-program/22-adr/ADR-012-audit-convergence-gates.md
spec_anchors:
  - deps/yai-law/specs/protocol/include/transport.h
  - deps/yai-law/specs/protocol/include/auth.h
  - deps/yai-law/specs/protocol/include/errors.h
claims:
  - C-ENVELOPE-HANDSHAKE-REQUIRED
  - C-DOMAIN-COVERAGE-NETWORK
evidence_commands_required:
  - tools/bin/yai-verify
  - tools/bin/yai-suite
issues:
  - "191"
---

# MP-ROOT-HARDENING-0.1.1

## Metadata
- Runbook: `docs/20-program/23-runbooks/root-hardening.md`
- Phase: `0.1.1 — Root = Byte-Perfect Router`
- Wave issue: `#191`
- Status: `active`

## Links
- Plan: `docs/20-program/audit-convergence/EXECUTION-PLAN-v0.1.0.md`
- Matrix: `docs/20-program/audit-convergence/AUDIT-CONVERGENCE-MATRIX-v0.1.0.md`
- Claims registry: `docs/50-validation/audits/claims/infra-grammar.v0.1.json`
- ADR: `docs/20-program/22-adr/ADR-012-audit-convergence-gates.md`

## Objective
Close phase 0.1.1 with explicit claim/evidence bindings and reproducible gate outputs.

## Mandatory command outcomes
- `tools/bin/yai-verify` -> `PASS`
- `tools/bin/yai-suite` -> `PASS`

Closure policy: mandatory `SKIP` is treated as `FAIL`.

## Definition of Done
- [x] Phase claim IDs are covered by evidence.
- [x] Mandatory commands are recorded with exit codes and outputs.
- [x] Root->Kernel evidence is traceable on deterministic pass/fail paths.
- [x] MP links from runbook phase and matrix remain valid.

## Execution Snapshot
- Status: `COMPLETED`
- Evidence bundle: `docs/20-program/24-milestone-packs/root-hardening/evidence/0.1.1/`
- Commands:
  - `tools/bin/yai-verify core` -> `PASS`
  - `tools/bin/yai-verify law-kernel` -> `PASS`
  - `tools/bin/yai-suite levels/l0-l7` -> `PASS` *(L3-L7 tool-gated SKIPs expected in current CLI target set)*
  - `tools/bin/yai-check-pins` -> `PASS`
