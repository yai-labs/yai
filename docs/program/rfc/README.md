---
role: support
status: active
audience: governance
owner_domain: program-rfc
id: RFC-INDEX
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
| PRP-001 | `rfc-runtime-001-runtime-topology-and-authority.md` | `docs/program/adr/adr-runtime-001-single-runtime.md` | draft |
| PRP-002 | `rfc-protocol-002-unified-rpc-and-cli-contract.md` | `docs/program/adr/adr-protocol-006-unified-rpc.md` | draft |
| PRP-003 | `rfc-workspace-003-workspace-lifecycle-and-isolation.md` | `docs/program/adr/adr-workspace-007-workspace-isolation.md` | draft |
| PRP-004 | `rfc-contracts-004-contract-runbook-lock-and-pin-policy.md` | `docs/program/adr/adr-contracts-011-contract-runbook-lock.md` | draft |
| PRP-005 | `rfc-formal-005-formal-coverage-roadmap.md` | `docs/program/adr/adr-program-012-audit-convergence-gates.md` | draft |

## Rules

- RFC numbering is preserved from PRP numbering (001..005).
- PRP IDs are retained only as historical mapping metadata.
- New proposal work should start directly as RFC.

# Related Docs
- `docs/program/rfc/README.md`
- Linked ADR and report artifacts
