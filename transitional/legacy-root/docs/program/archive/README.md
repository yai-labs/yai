---
role: support
status: active
audience: governance
owner_domain: program-archive
decision_id: PROGRAM-ARCHIVE
depends_on: [docs/program/README.md]
supersedes: []
superseded_by: []
implements: []
evidenced_by: []
related: [docs/archive/migration/d18.6-docs-freeze-readiness-audit.md]
---
# Program Archive

# Purpose
Store historical program artifacts de-promoted from the live governance surface.

# Scope
Contains milestone-pack history, retired report shards, and legacy delivery artifacts.

# Relationships
- `docs/program/README.md`
- `docs/archive/migration/d18.6-docs-freeze-readiness-audit.md`

# Canonical Role
Historical program memory only; non-authoritative for current platform behavior.

# Main Body
## What Is Archived Here
- `milestone-packs/`: tranche and delivery history.
- `reports/`: non-canonical report shards and deprecated report bundles.
- `legacy/`: additional retired program notes if needed.

## Why De-Promoted
Program history is preserved without competing with canonical live docs (`adr`, `rfc`, `policies`, `templates`, `reports`).

## Retrieval Rule
Use archive only for retrospective traceability and evidence continuity.

# Related Docs
- `docs/program/archive/legacy/decision-ledger.md`
- `docs/program/reports/README.md`
