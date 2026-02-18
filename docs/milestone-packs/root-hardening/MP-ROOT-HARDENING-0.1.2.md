Milestone Pack: `MP-ROOT-HARDENING-0.1.2`
Runbook link: `docs/runbooks/root-hardening.md` (phase `0.1.2 — Envelope-Only Authority Gate`)
Owner: runtime

Objective:
- Enforce privileged command authorization via envelope metadata only (`command_id`, `arming`, `role`, `ws_id`) in both Root and Kernel.

Contract Delta:
- Envelope: no new fields; enforcement semantics become strict for privileged commands.
- Authority: privileged commands require `arming=1` and `role>=operator`.
- Errors: deterministic rejects for missing arming / insufficient role.
- Logging: reject reason and code must be audit-visible at Root and Kernel boundaries.

Repo Split:
- `yai`: early reject in Root + defense-in-depth reject in Kernel with identical codes.
- `yai-cli`: expose explicit operator intent and provide reproducible positive/negative authority proofs.

Evidence Plan (minimum):
- Positive cases:
  - Privileged command with `arming=1` and `role=operator` succeeds.
  - Non-privileged command succeeds without elevated requirements.
- Negative cases:
  - Privileged command without arming returns deterministic `arming required` reject.
  - Privileged command with low role returns deterministic `role required` reject.

Compatibility Classification:
- Type: B
- Rationale: old client flows that do not provide compliant envelope authority metadata are rejected.
- Upgrade path: ship Twin PRs (`yai` + `yai-cli`) and merge in a coordinated window.

Definition of Done:
- [ ] Root fast-fails privileged non-compliant requests.
- [ ] Kernel rejects the same non-compliant requests (defense-in-depth).
- [ ] Error code semantics are identical in Root and Kernel paths.
- [ ] Twin PR evidence (pos/neg) is complete and reviewable.
