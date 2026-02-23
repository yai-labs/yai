# D5-economic/transaction-authorization-v1 - Economic Domain Pack (Transaction Authorization)

This pack defines the semantic meaning of forbidden transaction authorization in the Economic domain.

Runtime grammar invariant:
trigger -> context -> authority/contract -> decision -> enforcement -> evidence

## Forbidden effect (semantic)
- A transaction authorization is requested for an amount/counterparty not covered by an active contract, policy, or limit set.

## Runtime effect domain (where enforcement happens)
- `providers` (payment rails / bank gateway / PSP adapter)
- optionally `control` (approval/arming workflows)

## Intended use
- SC-102 (core-only) qualification: prove fail-closed authorization decisions + evidence completeness.
- Designed to be safe: no real payments are executed; vectors are simulated authorization requests.
