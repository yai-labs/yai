# D6-operational/incident-response-action-v1 - Operational Domain Pack (Incident Response Actions)

This pack defines the semantic meaning of forbidden operational actions in the Operational domain,
with a focus on incident response / SRE-style containment actions.

Runtime grammar invariant:
trigger -> context -> authority/contract -> decision -> enforcement -> evidence

## Forbidden effects (semantic)
- Executing a high-impact operational action (e.g., isolate host, revoke credentials, block egress)
  without required escalation and arming.
- Executing actions outside the allowed maintenance/response window.
- Executing actions in an unauthorized scope (e.g., production) without sufficient authority.

## Runtime effect domain (where enforcement happens)
- `control` (action request admission)
- `providers` (ops adapter, e.g., firewall/EDR/SOAR mock provider)

## Intended use
- SC-102 (core-only) qualification: prove fail-closed action governance + evidence completeness.
- Safe by design: vectors MUST be simulation-only; no real infra changes.
