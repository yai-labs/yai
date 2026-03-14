---
role: support
status: active
audience: governance
owner_domain: program-report
decision_id: RPT-INDEX
depends_on: [docs/program/README.md]
supersedes: []
superseded_by: []
implements: []
evidenced_by: []
related: [docs/program/reports/runtime-convergence-report.md,docs/program/reports/audit-convergence-report.md]
---
# Program Reports

# Purpose
Provides canonical program-level convergence evidence.

# Scope
Covers only active canonical program reports.

# Relationships
- `docs/program/README.md`
- Architecture and runbook canonical surfaces

# Canonical Role
Minimal live report spine.

# Main Body
Reports are compressed to canonical core.

## Navigation Order
1. `runtime-convergence-report.md`
2. `audit-convergence-report.md`

## Rules
- Do not add parallel report shards into live docs.
- Archive non-canonical report history under `docs/archive/**`.

# Related Docs
- `docs/program/README.md`


## Lifecycle Rules
- Every program core document must declare `decision_id` and `status`.
- Use statuses from D18.3 lifecycle grammar only (`draft`, `accepted`, `active`, `superseded`, `historical`).
- Superseded or historical docs must declare `superseded_by`.
- New ADR/RFC/MP/report entries must be added to `docs/program/archive/legacy/decision-ledger.md`.
