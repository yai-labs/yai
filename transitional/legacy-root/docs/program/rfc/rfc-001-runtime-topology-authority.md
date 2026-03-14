---
role: support
status: draft
audience: governance
owner_domain: program-rfc
id: RFC-001
decision_id: RFC-001
supersedes: []
superseded_by: []
implements: []
evidenced_by: [docs/program/reports/audit-convergence-report.md]
related: [docs/program/adr/adr-001-single-runtime.md,docs/program/adr/adr-003-kernel-authority.md]
---
# RFC-001 - Runtime topology and authority boundaries

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
## Problem
Architecture decisions around runtime scope and authority are spread across multiple ADRs. The project needs one pre-decision framing that explains why a machine-level runtime is preferred and how authority boundaries are enforced end-to-end.

## Scope
- In scope: Runtime topology options, authority plane boundaries, workspace-to-runtime routing model.
- Out of scope: Protocol wire-format details and CLI command taxonomy.

## Proposed Change
Define and document the rationale for one machine-level runtime with strict Root -> Kernel -> Engine authority flow and workspace isolation by design.

## Options Compared
- Option A: Single machine-level runtime with multi-workspace routing.
- Option B: Per-workspace daemon model with coordination layer.

## Risks
- Migration complexity from workspace-first habits. Mitigation: phased rollout through runbook phases.
- Confusion on authority ownership. Mitigation: explicit governance anchors and boundary diagrams.

## Rollout Sketch
1. Publish topology proposal with explicit L0 anchors.
2. Confirm ADR mapping set (001..005).
3. Gate implementation phases through root-hardening milestones.

## Exit Criteria
- [ ] Proposal clearly maps alternatives and selects a preferred topology.
- [ ] Governance anchors for authority boundaries are explicit and complete.
- [ ] Target ADR set is confirmed and linked.

## Traceability

- Spec anchors (if any): `../governance/foundation/axioms/A-002-authority.md`, `../governance/foundation/boundaries/L1-kernel.md`, `../governance/foundation/boundaries/L2-engine.md`
- Targets ADR: `docs/program/adr/adr-001-single-runtime.md`, `docs/program/adr/adr-002-root-entrypoint.md`, `docs/program/adr/adr-003-kernel-authority.md`, `docs/program/adr/adr-004-engine-execution.md`, `docs/program/adr/adr-005-mind-proposer.md`
- Downstream runbook: `docs/archive/legacy/program/milestone-packs/runtime-baselines/operations-foundation/mp-runtime-000-root-hardening.md`
- Downstream MP: `docs/archive/legacy/program/milestone-packs/root-hardening/mp-runtime-000-root-hardening-v0-1-5.md`

## References
- `docs/program/spine.md`
- `docs/program/traceability.md`
- `docs/program/adr/adr-001-single-runtime.md`

# Related Docs
- `docs/program/rfc/README.md`
- Linked ADR and report artifacts
