# Workspace Cross-Scope Misuse Model (WS-11)

Cross-scope misuse is any attempt to execute or inspect through a workspace different from the active binding context.

## Canonical Rule

Runtime control path is workspace-scoped:

- active binding defines authoritative workspace scope
- target workspace mismatch is rejected
- inspect/policy/debug surfaces resolve from active scope only

## Misuse Examples

- active workspace `A`, runtime call sent with envelope workspace `B`
- stale binding points to removed workspace
- tampered runtime root metadata attempts to redirect scope

## Expected Runtime Behavior

- reject with deterministic reason
- preserve active workspace boundary integrity
- avoid implicit fallback to unrelated workspace context
