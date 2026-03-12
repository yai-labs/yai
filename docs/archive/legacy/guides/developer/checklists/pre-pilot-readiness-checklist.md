# Pre-Pilot Readiness Checklist

Use this checklist before technical demo or stakeholder review.

## Build and runtime

- [ ] `make -j4 yai` succeeds
- [ ] no stale runtime socket/process blocks execution
- [ ] baseline integration scripts can start runtime and cleanup

## Governance object path

- [ ] explicit governance object selected for scenario
- [ ] attach step succeeds
- [ ] attachment visible in inspect/policy/debug operational state

## Workspace flow readability

- [ ] declared context visible
- [ ] event surface visible (`declared/business/enforcement`)
- [ ] operational state visible (`last_event`, `review_state`, `operational_summary`)

## Resolution/evidence path

- [ ] effective stack reference visible
- [ ] authority/evidence summaries visible
- [ ] trace reference visible in debug/inspect

## Repeatability

- [ ] vertical slice script passes: `workspace_governed_vertical_slice.sh`
- [ ] semantic split script passes: `workspace_event_surface_semantics.sh`
- [ ] flow-state readability script passes: `workspace_flow_state_readability.sh`

## Documentation coherence

- [ ] baseline anchor doc updated
- [ ] runbook steps match actual scripts
- [ ] expected-output guide matches current surfaces
- [ ] limits and non-goals are explicit

## Honest readiness gate

Proceed to pre-pilot discussion only if all critical checks above are green.
