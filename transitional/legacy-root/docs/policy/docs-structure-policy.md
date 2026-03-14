---
role: support
status: active
audience: maintainer
owner_domain: docs-policy
depends_on: [docs/README.md]
---
# Docs Structure Policy

## Canonical L1 Lock
Only these L1 docs families are allowed:
- `architecture`, `guides`, `reference`, `runbooks`, `program`, `policies`, `generated`, `archive`

No additional L1 macro-section is permitted without explicit policy revision.

## Subsection Creation Contract
Create a new subsection only if:
1. It maps to a stable documentation/system domain.
2. It materially improves navigation.
3. It cannot be solved by better file naming in current parent.
4. It has enough immediate scope to avoid decorative folders.

## Anti-Decoration Rules
- `README + 1 file` is not sufficient by default for a new subsection.
- Symmetry-only folder creation is forbidden.
- Depth should be minimized unless justified by stable domain boundaries.
