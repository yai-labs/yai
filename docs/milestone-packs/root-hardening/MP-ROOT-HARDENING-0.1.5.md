Milestone Pack: `MP-ROOT-HARDENING-0.1.5`
Runbook link: `docs/runbooks/root-hardening.md` (phase `0.1.5 — Test Matrix + Torture Suite`)
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
