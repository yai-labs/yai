---
role: support
status: active
audience: governance
owner_domain: program-adr
id: ADR-README
decision_id: ADR-INDEX
depends_on: [docs/program/README.md]
supersedes: []
superseded_by: []
implements: []
evidenced_by: []
related: [docs/program/README.md]
---
# ADR Index

# Purpose
Captures architecture decision records used for governance traceability.

# Scope
Covers decision context, accepted direction, and downstream implications.

# Relationships
- Related RFCs
- Associated implementation evidence and reports

# Canonical Role
Program support artifact with decision authority in governance context.

# Main Body
Architecture Decision Records (ADRs) capture irreversible or high-impact technical choices.

An ADR should answer:
- what was decided,
- why alternatives were rejected,
- what consequences are accepted,
- which foundation/spec anchors govern the decision.

## Canonical ADR set

- `adr-001-single-runtime.md`
- `adr-002-root-entrypoint.md`
- `adr-003-kernel-authority.md`
- `adr-004-engine-execution.md`
- `adr-005-mind-proposer.md`
- `adr-006-unified-rpc.md`
- `adr-007-workspace-isolation.md`
- `adr-008-connection-lifecycle.md`
- `adr-009-engine-attachment.md`
- `adr-010-boot-entrypoint.md`
- `adr-011-contract-runbook-lock.md`
- `adr-012-audit-convergence-gates.md`
- `adr-013-acquisition-control.md`
- `adr-014-secure-peering.md`
- `adr-015-daemon-architecture.md`
- `adr-016-global-edge-policy-hierarchy.md`
- `adr-017-edge-enforcement.md`
- `adr-018-runtime-observation.md`
- `adr-019-edge-binding-actions.md`
- `adr-020-workspace-authority-truth.md`
- `adr-021-workspace-policy-distribution.md`
- `adr-022-edge-policy-validity.md`
- `adr-023-governed-sovereign-mesh.md`
- `adr-024-mesh-discovery.md`
- `adr-025-mesh-coordination.md`
- `adr-026-sovereign-mesh-authority.md`
- `adr-027-secure-overlay-transport.md`
- `adr-028-owner-remote-peer-ingress.md`
- `adr-029-overlay-integration.md`
- `adr-030-source-edge-query-surfaces.md`
- `adr-031-unified-graph-runtime.md`
- `adr-032-ai-grounding-case-state.md`

## Template policy

The ADR template is canonical only in:
- `docs/program/templates/adr/ADR-000-template.md`

No template copy should be kept inside this ADR directory.

## Maintenance notes

When adding a new ADR:
- update this index,
- ensure foundation/spec references are explicit,
- add downstream runbook linkage when available.

Program-level convergence governance is defined in:
- `docs/program/reports/audit-convergence-report.md`

# Related Docs
- `docs/program/adr/README.md`
- Linked RFC/report artifacts


## Lifecycle Rules
- Every program core document must declare `decision_id` and `status`.
- Use statuses from D18.3 lifecycle grammar only (`draft`, `accepted`, `active`, `superseded`, `historical`).
- Superseded or historical docs must declare `superseded_by`.
- New ADR/RFC/MP/report entries must be added to `docs/program/archive/legacy/decision-ledger.md`.
