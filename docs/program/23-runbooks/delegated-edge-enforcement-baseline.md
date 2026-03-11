# Delegated Edge Enforcement Baseline (RF-0.3)

## Goal

Provide baseline operator semantics for delegated local enforcement behavior on
`yai-daemon`.

## Local Modes

- observe-only
- post-event local enforcement
- preventive local enforcement
- escalated enforcement

## Local Outcomes

- `observe_only`
- `allow`
- `block`
- `hold`
- `execute`
- `escalate`
- `defer`
- `deny_due_to_missing_scope`
- `deny_due_to_expired_grant`

## Guardrails

- local enforcement is valid only under owner-issued scope/grants
- stale/missing/expired delegation reduces autonomy
- hold/escalate path is preferred over sovereign local adjudication
- owner remains final policy/conflict authority
