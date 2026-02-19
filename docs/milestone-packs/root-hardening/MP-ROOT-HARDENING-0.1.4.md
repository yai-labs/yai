---
id: MP-ROOT-HARDENING-0.1.4
status: active
runbook: docs/runbooks/root-hardening.md
phase: "0.1.4 — Kernel Hard Reject on Invalid ws_id"
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
Milestone Pack: `MP-ROOT-HARDENING-0.1.4`
Runbook link: `docs/runbooks/root-hardening.md` (phase `0.1.4 — Kernel Hard Reject on Invalid ws_id`)
ADR links: `docs/design/adr/ADR-002-root-entrypoint.md`, `docs/design/adr/ADR-006-unified-rpc.md`, `docs/design/adr/ADR-008-connection-lifecycle.md`, `docs/design/adr/ADR-011-contract-baseline-lock.md`
Proposal links: `docs/design/proposals/PRP-001-runtime-topology-and-authority.md`, `docs/design/proposals/PRP-002-unified-rpc-and-cli-contract.md`, `docs/design/proposals/PRP-004-contract-baseline-lock-and-pin-policy.md`, `docs/design/proposals/PRP-005-formal-coverage-roadmap.md`
Evidence links: `docs/test-plans/hardfail.md`

Owner: runtime

Objective:
- Ensure invalid `ws_id` causes deterministic Kernel reject with zero session/filesystem side effects.

Contract Delta:
- Envelope: none.
- Authority: none.
- Errors: deterministic invalid `ws_id` error response frame from Kernel.
- Logging: reject path must expose reason and trace context.

Repo Split:
- `yai`: enforce hard reject before session/dir creation in Kernel paths.
- `yai-cli`: optional negative harness vectors for invalid ws_id filesystem side-effect checks.

Evidence Plan (minimum):
- Positive cases:
  - Valid ws_id creates/uses session flow normally.
  - Valid ws_id command path remains behaviorally unchanged from previous milestone.
- Negative cases:
  - Empty/invalid ws_id returns deterministic error frame.
  - Invalid ws_id attempt produces no `~/.yai/run/<ws_id>` side effects.

Compatibility Classification:
- Type: A
- Rationale: hard reject only applies to invalid values that are already non-conformant.
- Upgrade path: conformant clients unchanged.

Definition of Done:
- [ ] Kernel never creates session or dirs when ws_id is invalid.
- [ ] Kernel always replies with deterministic error frame on invalid ws_id.
- [ ] Side-effect assertions are covered in automated checks.
- [ ] PR evidence includes filesystem assertions and logs.
