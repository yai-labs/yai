---
role: support
status: active
audience: maintainer
owner_domain: docs-policy
depends_on: [docs/README.md]
---
# Archive Transfer Policy

## Mandatory De-Promotion Cases
Move from live to archive when a doc is:
- superseded by a canonical spine document,
- historical tranche/refactor/migration material,
- intermediate report or milestone artifact,
- temporary cutover guidance no longer active,
- replaced by merged/condensed canonical content.

## Archive Destination Rules
- Program historical artifacts -> `docs/program/archive/**`
- Global historical/migration artifacts -> `docs/archive/**`

## Delete vs Archive
- Archive when traceability/compliance value exists.
- Delete when fully redundant and no retrospective value remains.

## Transfer Requirements
When de-promoting, update links/readmes/indexes to ensure no live-navigation dependency remains on transferred docs.
