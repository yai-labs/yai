# Workspace Policy Inspection (WS-3)

WS-3 introduces inspectability primitives so developers can query workspace state before full scenario execution.

## Typical Flow

1. Activate workspace
2. `yai.workspace.current`
3. `yai.workspace.status`
4. `yai.workspace.domain_get`
5. Optional: `yai.workspace.domain_set` (declared context)
6. Execute runtime control call
7. `yai.workspace.policy_effective`
8. `yai.workspace.debug_resolution`
9. `yai.workspace.inspect`

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
- effective stack references
- effect summary
- trace reference

Use `workspace.inspect` for full workspace-level state inspection.
