# Workspace Policy Inspection (WS-3)

WS-3 introduces inspectability primitives so developers can query workspace state before full scenario execution.

## Typical Flow

1. Set workspace
2. `yai.workspace.current`
3. `yai.workspace.status`
4. `yai.workspace.domain_get`
5. Optional: `yai.workspace.domain_set` (declared context)
6. Optional: `yai.workspace.policy_dry_run` (eligibility/compatibility preview)
7. Optional: `yai.workspace.policy_attach` (explicit governable object attachment)
8. Optional: `yai.workspace.policy_activate` (explicit activation)
9. Execute runtime action (`yai.workspace.run <action> [tokens...]`)
10. `yai.workspace.policy_effective`
11. `yai.workspace.debug_resolution`
12. `yai.workspace.inspect`

## What Is Reliable in This Phase

Reliable now:
- current active workspace binding state
- declared context read/write and validation
- inferred/effective summaries after resolution calls
- basic authority/evidence/effect summaries on inspect/policy/debug surfaces

Not finalized yet:
- final CLI UX/presentation
- full workspace scenario e2e workflows
- advanced shell prompt integration

## Domain Context Rules

`domain_set` updates only declared context.

- declared family and specialization are validated against embedded canonical indices
- inferred/effective context remains runtime-owned
- if family is changed without specialization and current specialization becomes incompatible, specialization is cleared

## Debug Use

Use `workspace.debug_resolution` to see the latest compact resolution rationale inputs:
- declared/inferred context
- event surface (`declared_scenario_specialization`, `business_specialization`, `enforcement_specialization`, `flow_stage`)
- effective stack references
- effect summary
- trace reference

Use `workspace.inspect` for full workspace-level state inspection.

DP-5 adds `governance_persistence` references in inspect/effective/debug
surfaces, pointing to persisted governance object/lifecycle/attachment stores.

DP-6 adds `authority_artifact_persistence` references in inspect/effective/debug
surfaces, pointing to persisted authority state/resolution and artifact metadata/linkage stores.

DP-7 adds `brain_persistence` references in inspect/effective/debug surfaces,
separating persistent graph truth (`graph_truth_authoritative=true`) from
transient cognition heat (`transient_authoritative=false`).


Operational-state checks during inspection:

- verify `attached_governance_objects` against expected attachments
- verify `last_event_ref` and `last_flow_stage`
- verify `last_business_specialization` vs `last_enforcement_specialization`
- verify `review_state` and `operational_summary`
