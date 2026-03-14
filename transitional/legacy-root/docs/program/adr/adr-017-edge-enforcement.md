---
role: support
status: accepted
audience: governance
owner_domain: program-adr
id: ADR-017
decision_id: ADR-017
supersedes: []
superseded_by: []
implements: []
evidenced_by: [docs/program/reports/runtime-convergence-report.md]
related: []
anchor: "#delegated-edge-enforcement-model"
applies_to: 
effective_date: 2026-03-11
phase: RF-0.3
---
# ADR-017 - Delegated Edge Enforcement Model

## Context

RF-0.1 and RF-0.2 locked daemon role and policy hierarchy. We now need explicit
local enforcement semantics to avoid ambiguity between monitoring and action.

## Decision

YAI adopts delegated edge enforcement model with explicit levels:

- observe-only
- post-event local enforcement
- preventive local enforcement
- escalated enforcement

and canonical local outcomes:

- `observe_only`, `allow`, `block`, `hold`, `execute`, `escalate`, `defer`
- `deny_due_to_missing_scope`, `deny_due_to_expired_grant`

## Authority Rule

Local enforcement is delegated execution, not sovereign adjudication.

- edge actions are valid only under owner-issued scope/grants
- edge cannot create new policy authority
- owner remains final authority for workspace truth and conflict truth

## Stale/Failure Rule

Stale/missing/expired delegated state must reduce autonomy, never expand it.

## Consequences

### Positive

- clear semantics for action mediation and qualification tests
- strong boundary against edge sovereignty drift
- explicit hold/escalate path for uncertain cases

### Negative

- requires conservative fallback behavior under degraded conditions
- full runtime implementation remains follow-up work

## Non-goals

- full delegated enforcement implementation
- full grant lifecycle implementation
- disconnected sovereign edge operation

## References

- `docs/architecture/delegated-edge-enforcement-model.md`
- `docs/architecture/global-to-edge-policy-hierarchy-model.md`
