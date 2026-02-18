Milestone Pack: `MP-ROOT-HARDENING-0.1.4`
Runbook link: `docs/runbooks/root-hardening.md` (phase `0.1.4 — Kernel Hard Reject on Invalid ws_id`)
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
