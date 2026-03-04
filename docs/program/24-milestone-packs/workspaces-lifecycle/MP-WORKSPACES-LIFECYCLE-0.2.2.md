---
id: MP-WORKSPACES-LIFECYCLE-0.2.2
status: draft
runbook: docs/program/23-runbooks/workspaces-lifecycle.md
phase: "0.2.2 — bundle command wiring wave"
owners:
  - runtime
adrs:
  - docs/program/22-adr/ADR-006-unified-rpc.md
  - docs/program/22-adr/ADR-007-workspace-isolation.md
  - docs/program/22-adr/ADR-011-contract-baseline-lock.md
  - docs/program/22-adr/ADR-012-audit-convergence-gates.md
spec_anchors:
  - yai-law/registry/commands.v1.json
  - docs/program/23-runbooks/workspaces-lifecycle-command-map.v2.md
target_group: bundle
target_command_count: 200
---

# MP-WORKSPACES-LIFECYCLE-0.2.2

## Objective
Plan and execute real runtime wiring for group `bundle` without contract drift.

Group mission: Bundle assembly and package contract surfaces.

## Scope (Planned)
- Canonical target group: `bundle`
- Canonical command count: `200`
- Family distribution (top): `channel_*` (20), `index_*` (20), `manifest_*` (20), `package_*` (20), `payload_*` (20), `proof_*` (20), `release_*` (20), `signature_*` (20), `target_*` (20), `policy_*` (19)
- Delivery model: keep all registered commands invocable; implement selected handlers first; missing handlers remain deterministic (`nyi` equivalent).

## Representative command_id set
- `yai.bundle.bundle`
- `yai.bundle.channel_attach`
- `yai.bundle.channel_audit`
- `yai.bundle.channel_build`
- `yai.bundle.channel_detach`
- `yai.bundle.channel_digest`
- `yai.bundle.channel_export`
- `yai.bundle.channel_import`
- `yai.bundle.channel_inspect`
- `yai.bundle.channel_lint`
- `yai.bundle.channel_merge`
- `yai.bundle.channel_promote`
- `yai.bundle.channel_publish`
- `yai.bundle.channel_repack`
- `yai.bundle.channel_rollback`

## Definition of Done
- [ ] Group `bundle` commands remain discoverable in CLI help.
- [ ] No `unknown command` for registered IDs in this group.
- [ ] Selected real handlers are wired end-to-end (CLI -> SDK -> Root -> Kernel/Engine).
- [ ] Non-implemented commands return deterministic error model (`ok/error/nyi` mapping).
- [ ] Evidence pointers/logs for executed commands are archived.

## Required evidence (to fill at execution time)
- [ ] Build/test logs for touched repos.
- [ ] Command execution matrix for this group.
- [ ] Runtime logs with deterministic outcomes.
- [ ] Audit mapping updates (claim-by-claim if Gate A scope is impacted).

## Risks and follow-ups
- Risk: wide family surface may hide semantic collisions.
- Mitigation: implement by family slices, keeping contract fixed.
- Follow-up: chain next MP after this group reaches stable deterministic behavior.

## Closure decision
Status: `PLANNED` (no runtime implementation claimed in this MP).
