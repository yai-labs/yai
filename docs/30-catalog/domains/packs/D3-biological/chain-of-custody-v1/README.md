# D3-biological/chain-of-custody-v1 - Biological Domain Pack (Consent & Chain-of-Custody)

This pack defines the semantic meaning of forbidden biological sample/data access under consent and chain-of-custody constraints.

Runtime grammar invariant:
trigger -> context -> authority/contract -> decision -> enforcement -> evidence

## Forbidden effects (semantic)
- Reading a biological sample record or derived dataset without valid consent.
- Accessing a sample when the chain-of-custody is incomplete or broken.
- Exporting sample data for an undeclared purpose.

## Runtime effect domain (where enforcement happens)
- `storage` (sample/dataset reads and exports)
- optionally `providers` (lab system adapter) and `control` (approval/arming)

## Intended use
- SC-102 (core-only) qualification: prove fail-closed on missing consent/custody + evidence completeness.
- Safe by design: vectors use simulated sample IDs and synthetic metadata (no real patient data).
