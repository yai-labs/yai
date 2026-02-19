---
id: MP-ROOT-HARDENING-0.1.0
status: active
runbook: docs/runbooks/root-hardening.md
phase: "0.1.0 — Protocol Guardrails"
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
Milestone Pack: `MP-ROOT-HARDENING-0.1.0`
Runbook link: `docs/runbooks/root-hardening.md` (phase `0.1.0 — Protocol Guardrails`)
ADR links: `docs/design/adr/ADR-002-root-entrypoint.md`, `docs/design/adr/ADR-006-unified-rpc.md`, `docs/design/adr/ADR-008-connection-lifecycle.md`, `docs/design/adr/ADR-011-contract-baseline-lock.md`
Proposal links: `docs/design/proposals/PRP-001-runtime-topology-and-authority.md`, `docs/design/proposals/PRP-002-unified-rpc-and-cli-contract.md`, `docs/design/proposals/PRP-004-contract-baseline-lock-and-pin-policy.md`, `docs/design/proposals/PRP-005-formal-coverage-roadmap.md`
Evidence links: `docs/test-plans/hardfail.md`

Owner: runtime

Objective:
- Root and Kernel enforce the same mechanical envelope invariants and deterministic protocol rejects.

Contract Delta:
- Envelope: none (strict enforcement of existing fields/invariants only).
- Authority: none.
- Errors: use spec-defined numeric codes only.
- Logging: reject path must remain observable and auditable.

Repo Split:
- `yai`: enforce guardrails at Root and Kernel decode boundaries.
- `yai-cli`: no required wire change; optional negative harness vectors to lock behavior.

Evidence Plan (minimum):
- Positive cases:
  - Valid handshake and `yai root ping` succeed end-to-end.
  - Valid envelope at max-safe limits (payload length and enum ranges) succeeds.
- Negative cases:
  - Wrong `magic` and wrong `version` both return deterministic error frames.
  - Invalid `ws_id` and `payload_len > YAI_MAX_PAYLOAD` both return deterministic reject codes.

Compatibility Classification:
- Type: A
- Rationale: no protocol redesign; only stricter validation of already-invalid inputs.
- Upgrade path: existing conformant CLI behavior remains valid.

Definition of Done:
- [ ] Root and Kernel reject invalid frames with the same spec numeric codes.
- [ ] No silent drops on malformed inputs (always deterministic response frame).
- [ ] Baseline boot and `yai root ping` remain green.
- [ ] Evidence is captured in PR and CI logs.
