# Delegated Edge Enforcement Model (RF-0.3)

Status: active  
Owner: runtime  
Effective date: 2026-03-11

## Purpose

Define canonical delegated edge enforcement semantics for `yai-daemon` under
owner workspace sovereignty.

## Core Distinctions

- observation is not enforcement
- local evaluation is not final authority
- local action is delegated execution
- hold/escalation is not sovereign adjudication

## Enforcement Levels

1. **observe-only**
   - collect signals/evidence/status
   - no action effect on local action point

2. **post-event local enforcement**
   - react after event happened (contain/isolate/suspend secondary flow)
   - may escalate with context bundle

3. **preventive local enforcement**
   - gate action point before full execution
   - may return `allow`, `block`, `hold`, `execute`, `defer`

4. **escalated enforcement**
   - freeze/hold/defer locally
   - request owner evaluation
   - apply owner instruction when available

## Canonical Local Outcomes (baseline)

- `observe_only`
- `allow`
- `block`
- `hold`
- `execute`
- `escalate`
- `defer`
- `deny_due_to_missing_scope`
- `deny_due_to_expired_grant`

## Authority Boundary

Local enforcement is delegated execution, not sovereign adjudication.

Non-negotiable:

- edge actions require valid owner-issued scope/grants
- edge cannot expand policy scope autonomously
- edge cannot finalize policy/graph/conflict truth
- owner remains final workspace authority

## Stale/Missing Delegation Behavior

When snapshot/grant/capability state is stale, missing, ambiguous or expired:

- autonomy must be reduced
- never expanded
- fallback to `observe_only` or conservative deny/hold per grant contract
- escalation to owner is preferred over local sovereign decisions

## Integration Contract

This model is consumed by:

- runtime mediation (`exec`) for source-plane action points
- law attachability/governance constraints
- qualification scenarios for hold/escalate/stale grant behavior

## References

- `docs/architecture/global-to-edge-policy-hierarchy-model.md`
- `docs/architecture/source-plane-model-refoundation-rf01.md`
- `docs/program/22-adr/ADR-017-delegated-edge-enforcement-model.md`
