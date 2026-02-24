# D4-social-institutional/procedural-authorization-v1 - Social/Institutional Domain Pack (Procedural Authorization)

This pack defines the semantic meaning of forbidden procedural actions in the Social/Institutional domain.

Runtime grammar invariant:
trigger -> context -> authority/contract -> decision -> enforcement -> evidence

## Forbidden effects (semantic)
- Executing a procedural step without the required role, delegation, or arming state.
- Executing a step out of order (when the procedure requires sequencing).

## Runtime effect domain (where enforcement happens)
- `control` (procedure-step execution requests)
- optionally `storage` (writing an official act/record without authorization)

## Intended use
- SC-102 (core-only) qualification: prove fail-closed procedural enforcement + evidence completeness.
- Safe by design: vectors are simulated procedure-step requests.
