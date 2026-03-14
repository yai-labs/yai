---
role: support
status: active
audience: governance
owner_domain: program-mp
decision_id: MP-INDEX
depends_on: [docs/program/archive/legacy/decision-ledger.md]
supersedes: []
superseded_by: []
implements: []
evidenced_by: []
related: [docs/program/reports/README.md]
---
# Milestone Packs (Index-Only Live)

## Scope
Index-only live entry for milestone-pack history.

## What Belongs Here
- Pointers to archived milestone-pack families.

## What Does Not Belong Here
- Detailed milestone-pack content in live docs.

## Navigation Order
1. `docs/archive/legacy/program/milestone-packs/`
2. `docs/program/reports/README.md`

## Extension Rules
- New milestone-pack details are archived by default.
- Keep this section as index-only in live docs.


## Lifecycle Rules
- Every program core document must declare `decision_id` and `status`.
- Use statuses from D18.3 lifecycle grammar only (`draft`, `accepted`, `active`, `superseded`, `historical`).
- Superseded or historical docs must declare `superseded_by`.
- New ADR/RFC/MP/report entries must be added to `docs/program/archive/legacy/decision-ledger.md`.
