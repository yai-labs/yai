# Legacy Workspace Tests (Transitional Only)

`tests/legacy/workspace/*` is fenced legacy material.

These suites preserve historical workspace-era behavior for regression
comparison, but do not define the canonical YAI model.

## Canonical replacement model

- old center: workspace
- canonical center now: container domain (`sys/container/*`)

## Mapping bridge (legacy -> target)

- workspace containment flows -> container domain containment flows (`sys/container`, `kernel/container`)
- workspace runtime contract -> container/system runtime contract (`sys/container/runtime`)
- workspace session binding -> container session binding (`sys/container/session`)
- workspace state/recovery -> container state/recovery (`sys/container/runtime`, `sys/container/recovery`)
- workspace query/inspect surfaces -> `sys/container` + `sys/data` + `sys/graph`

## Lifecycle policy for these tests

- preserve temporarily: high-signal regression scripts still used in CI/manual qualification
- migrate: scripts that validate behavior now owned by container/system planes
- deprecate: scripts overlapping with canonical container suites
- demolish: scripts asserting workspace as architecture center

## Hard guardrails

- no new workspace-centered primary semantics
- no new test files should be added here unless explicitly tagged transitional
- every retained script should have a migration target or retirement decision
