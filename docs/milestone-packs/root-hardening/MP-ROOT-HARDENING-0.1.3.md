Milestone Pack: `MP-ROOT-HARDENING-0.1.3`
Runbook link: `docs/runbooks/root-hardening.md` (phase `0.1.3 — ws_id Validation Centralization`)
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
