# D7-cognitive-cultural/publication-enforcement-v1 - Cognitive/Cultural Domain Pack (Publication & Enforcement)

This pack defines the semantic meaning of forbidden publication/visibility changes in the Cognitive/Cultural domain.

Runtime grammar invariant:
trigger -> context -> authority/contract -> decision -> enforcement -> evidence

## Forbidden effects (semantic)
- Publishing content without required provenance.
- Changing visibility/ranking without policy basis.
- Enforcing moderation actions without contestability metadata (appeal path/decision trace).

## Runtime effect domain (where enforcement happens)
- `providers` (content platform adapter/publishing provider)
- optionally `storage` (writing immutable provenance records)
- optionally `control` (high-impact actions requiring arming)

## Intended use
- SC-102 (core-only) qualification: prove fail-closed publication governance + evidence completeness.
- Safe by design: vectors are simulated content actions, no real public publishing.
