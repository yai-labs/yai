# Workspace Runtime Routing by Containment (WS-10)

Runtime routing is workspace-scoped and now execution-mode-aware.

## Routing Inputs

- workspace binding (`set/switch/unset`)
- namespace validity
- containment readiness
- execution profile (requested/effective/degraded)

## Routing Outcomes

- Control calls are bound to active workspace namespace.
- Cross-workspace calls are denied by scope guard.
- Runtime surfaces expose:
  - requested/effective execution mode,
  - degraded reason,
  - unsupported scopes summary.

## Degraded Routing

If requested mode cannot be fully enforced:

- routing remains workspace-scoped,
- effective mode is downgraded,
- downgrade is visible and auditable via status/inspect/debug/policy.
