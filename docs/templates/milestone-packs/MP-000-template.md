Milestone Pack: `MP-<TRACK>-<PHASE>`
Runbook link: `docs/runbooks/<runbook>.md`
Owner: <team/role>

Objective:
- <single sentence invariant/outcome>

Contract Delta:
- Envelope: <none | fields/semantics changed>
- Authority: <none | new/changed rule>
- Errors: <codes/semantics>
- Logging: <required additions>

Repo Split:
- `yai`: <enforcement/routing/logging changes>
- `yai-cli`: <conformance/UX/harness changes>

Evidence Plan (minimum):
- Positive cases:
  - <case 1>
  - <case 2>
- Negative cases:
  - <case 1>
  - <case 2>

Compatibility Classification:
- Type: A | B
- Rationale: <why>
- Upgrade path: <how compatibility is handled>

Definition of Done:
- [ ] Core invariant enforced deterministically
- [ ] CLI proves invariant (positive + negative)
- [ ] Evidence captured and reviewable
- [ ] Compatibility notes updated
