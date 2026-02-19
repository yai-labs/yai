---
id: MP-ROOT-HARDENING-0.1.3
status: active
runbook: docs/runbooks/root-hardening.md
phase: "0.1.3 — ws_id Validation Centralization"
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
Milestone Pack: `MP-ROOT-HARDENING-0.1.3`
Runbook link: `docs/runbooks/root-hardening.md` (phase `0.1.3 — ws_id Validation Centralization`)
ADR links: `docs/design/adr/ADR-002-root-entrypoint.md`, `docs/design/adr/ADR-006-unified-rpc.md`, `docs/design/adr/ADR-008-connection-lifecycle.md`, `docs/design/adr/ADR-011-contract-baseline-lock.md`
Proposal links: `docs/design/proposals/PRP-001-runtime-topology-and-authority.md`, `docs/design/proposals/PRP-002-unified-rpc-and-cli-contract.md`, `docs/design/proposals/PRP-004-contract-baseline-lock-and-pin-policy.md`, `docs/design/proposals/PRP-005-formal-coverage-roadmap.md`
Evidence links: `docs/test-plans/hardfail.md`

Owner: runtime

Objective:
- Eliminate ws_id validation drift by enforcing one shared validator definition across Root, Kernel, and CLI.

Contract Delta:
- Envelope: none.
- Authority: none.
- Errors: invalid `ws_id` rejects remain deterministic and spec-consistent.
- Logging: invalid `ws_id` reject reason remains visible in audit logs.

Repo Split:
- `yai`: consume the canonical validator in Root and Kernel paths.
- `yai-cli`: apply the same validator before send while preserving server-side enforcement.

Evidence Plan (minimum):
- Positive cases:
  - Valid ws_id values pass in CLI, Root, and Kernel.
  - Boundary valid ws_id length (35 chars) is accepted end-to-end.
- Negative cases:
  - Invalid characters (`/`, `~`, whitespace) are blocked client-side and rejected server-side.
  - Overflow length (`36+`) is rejected deterministically and never reaches dispatch.

Compatibility Classification:
- Type: B
- Rationale: server-side strict centralization without synchronized CLI checks can create behavior drift for operators.
- Upgrade path: coordinated Twin PR rollout and explicit compatibility note in PRs.

Definition of Done:
- [ ] Only one ws_id validator definition remains authoritative.
- [ ] Root and Kernel consume that definition (no local divergent logic).
- [ ] CLI applies the same constraints before send.
- [ ] Evidence demonstrates same pass/fail outcomes across all three surfaces.
