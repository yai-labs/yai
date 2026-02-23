# D2-physical/actuator-command-v1 - Physical Domain Pack (Actuator Command Authorization)

This pack defines the semantic meaning of forbidden actuator commands in the Physical domain.

Runtime grammar invariant:
trigger -> context -> authority/contract -> decision -> enforcement -> evidence

## Forbidden effects (semantic)
- An actuator command is issued without an active authority contract for the workspace/principal.
- An actuator command is issued outside an allowed safety window (interlock/safety constraints).

## Runtime effect domain (where enforcement happens)
- `providers` (actuator adapter/driver provider)
- optionally `control` (arming/escalation gating)

## Intended use
- SC-102 (core-only) qualification via simulated actuator provider.
- Designed to be safe: vectors MUST be executed against a mock/sim adapter, not real hardware.
