---
role: support
status: active
audience: governance
owner_domain: program-rfc
id: RFC-INDEX
decision_id: RFC-INDEX
depends_on: [docs/program/README.md]
supersedes: []
superseded_by: []
implements: []
evidenced_by: []
related: [docs/program/README.md]
---
# RFC Series (Canonical)

# Purpose
Captures governance-level request-for-comment context and decisions.

# Scope
Covers rationale, constraints, and acceptance direction for platform evolution.

# Relationships
- Related ADRs
- Associated implementation evidence and reports

# Canonical Role
Program support artifact; not a runtime architecture source-of-truth.

# Main Body
This directory is the canonical home for governance RFCs migrated from legacy PRP proposals.

## Mapping PRP -> RFC

| Legacy PRP | Canonical RFC | Linked ADR | Status |
|---|---|---|---|
| PRP-001 | `rfc-001-runtime-topology-authority.md` | `docs/program/adr/adr-001-single-runtime.md` | draft |
| PRP-002 | `rfc-002-unified-rpc-cli-contract.md` | `docs/program/adr/adr-006-unified-rpc.md` | draft |
| PRP-003 | `rfc-003-workspace-lifecycle-isolation.md` | `docs/program/adr/adr-007-workspace-isolation.md` | draft |
| PRP-004 | `rfc-004-lock-pin-policy.md` | `docs/program/adr/adr-011-contract-runbook-lock.md` | draft |
| PRP-005 | `rfc-005-formal-coverage-roadmap.md` | `docs/program/adr/adr-012-audit-convergence-gates.md` | draft |

## Rules

- RFC numbering is preserved from PRP numbering (001..005).
- PRP IDs are retained only as historical mapping metadata.
- New proposal work should start directly as RFC.

# Related Docs
- `docs/program/rfc/README.md`
- Linked ADR and report artifacts


## Lifecycle Rules
- Every program core document must declare `decision_id` and `status`.
- Use statuses from D18.3 lifecycle grammar only (`draft`, `accepted`, `active`, `superseded`, `historical`).
- Superseded or historical docs must declare `superseded_by`.
- New ADR/RFC/MP/report entries must be added to `docs/program/archive/legacy/decision-ledger.md`.
