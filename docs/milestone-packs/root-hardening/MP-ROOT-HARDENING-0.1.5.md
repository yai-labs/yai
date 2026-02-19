---
id: MP-ROOT-HARDENING-0.1.5
status: active
runbook: docs/runbooks/root-hardening.md
phase: "0.1.5 — Test Matrix + Torture Suite"
adrs:
  - docs/design/adr/ADR-002-root-entrypoint.md
  - docs/design/adr/ADR-006-unified-rpc.md
  - docs/design/adr/ADR-008-connection-lifecycle.md
  - docs/design/adr/ADR-011-contract-baseline-lock.md
spec_anchors:
  - deps/yai-specs/specs/protocol/include/transport.h
  - deps/yai-specs/specs/protocol/include/auth.h
  - deps/yai-specs/specs/protocol/include/errors.h
issues:
  - N/A
issue_reason: "Docs-only traceability alignment PR without dedicated issue."
---
Milestone Pack: `MP-ROOT-HARDENING-0.1.5`
Runbook link: `docs/runbooks/root-hardening.md` (phase `0.1.5 — Test Matrix + Torture Suite`)
ADR links: `docs/design/adr/ADR-002-root-entrypoint.md`, `docs/design/adr/ADR-006-unified-rpc.md`, `docs/design/adr/ADR-008-connection-lifecycle.md`, `docs/design/adr/ADR-011-contract-baseline-lock.md`
Proposal links: `docs/design/proposals/PRP-001-runtime-topology-and-authority.md`, `docs/design/proposals/PRP-002-unified-rpc-and-cli-contract.md`, `docs/design/proposals/PRP-004-contract-baseline-lock-and-pin-policy.md`, `docs/design/proposals/PRP-005-formal-coverage-roadmap.md`
Evidence links: `docs/test-plans/hardfail.md`

Owner: runtime

Objective:
- Deliver a repeatable torture suite proving root hardening invariants with deterministic PASS/FAIL and audit-grade traces.

Contract Delta:
- Envelope: none.
- Authority: none.
- Errors: all negative vectors map to deterministic expected codes/semantics.
- Logging: all rejects are traceable in Root and Kernel logs.

Repo Split:
- `yai`: maintain server-side test hooks, gate commands, and deterministic error behavior.
- `yai-cli`: maintain operator-facing harness/suite execution surface and reporting for protocol vectors.

Evidence Plan (minimum):
- Positive cases:
  - Handshake ok and ping valid ws pass in sequence.
  - Privileged command with valid arming/role passes.
- Negative cases:
  - Wrong magic/version, payload overflow, bad checksum all fail with expected deterministic codes.
  - Invalid/missing ws_id and authority violations fail deterministically and leave auditable logs.

Compatibility Classification:
- Type: B
- Rationale: cross-repo harness parity is required to prove and operate the full hardening gate set.
- Upgrade path: coordinated Twin PR merge and shared release evidence before final tag.

Definition of Done:
- [ ] Torture suite runs as a single command and reports PASS/FAIL per vector.
- [ ] All mandatory vectors from runbook are covered and deterministic.
- [ ] Evidence links command output with Root/Kernel log traces.
- [ ] Twin PR record includes final cross-repo evidence and compatibility note.
