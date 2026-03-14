---
role: support
status: active
audience: maintainer
owner_domain: docs-policy
depends_on: [docs/README.md]
---
# Live Docs Admission Policy

## Admission Conditions
A new live document is admitted only if all conditions are true:
1. Belongs unambiguously to one canonical live family.
2. Does not duplicate an existing canonical owner doc.
3. Is not historical/intermediate progress material.
4. Is not a migration note, relocation map, or working note.
5. Uses policy-compliant naming.
6. Has justified live status and clear section ownership.

## Required Classification
Allowed live roles: `canonical`, `support`, `procedural`, `reference`.

## Required Metadata for High-Visibility Docs
- `role`
- `status`
- `owner_domain`
- `depends_on` (required when role is non-canonical)

## Rejection Rules
Reject from live if doc is:
- tranche/wave progress artifact,
- closeout/refoundation note,
- migration-only map,
- redundant with existing canonical spine.

## Archive-First Cases
Send to archive when content is historical, superseded, transitional, or retained only for retrospective traceability.
