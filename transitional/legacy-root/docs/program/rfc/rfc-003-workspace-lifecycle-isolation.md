---
role: support
status: draft
audience: governance
owner_domain: program-rfc
id: RFC-003
decision_id: RFC-003
supersedes: []
superseded_by: []
implements: []
evidenced_by: [docs/program/reports/audit-convergence-report.md]
related: [docs/program/adr/adr-007-workspace-isolation.md,docs/program/adr/adr-008-connection-lifecycle.md]
---
# RFC-003 - Workspace lifecycle and isolation guarantees

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
Workspace lifecycle behavior is currently captured at ADR level but lacks a consolidated pre-decision analysis that spans lock, connection lifecycle, boot/attach flow, and runtime routing.

## Scope
- In scope: Workspace lock semantics, connection lifecycle, attach/boot boundaries, routing constraints.
- Out of scope: Protocol envelope fields and formal-coverage expansion.

## Proposed Change
Define one lifecycle proposal that aligns isolation guarantees with Kernel boundary rules and explicitly traces lifecycle states to contract invariants.

## Options Compared
- Option A: Lockfile-first lifecycle with incremental runtime registry transition.
- Option B: Immediate runtime-registry lifecycle with lockfiles deprecated.

## Risks
- Race conditions in transition path. Mitigation: explicit state machine and deterministic reject tests.
- Operator confusion during mixed mode. Mitigation: docs and runbook acceptance checks.

## Rollout Sketch
1. Define lifecycle states and invalid transitions.
2. Map each transition to spec/invariant anchors.
3. Gate rollout with workspace lifecycle runbook phases.

## Exit Criteria
- [ ] Lifecycle state model is documented with valid/invalid transitions.
- [ ] Isolation guarantees are mapped to boundary and invariant anchors.
- [ ] ADR mapping set (007..010) is explicit.

## Traceability

- Spec anchors (if any): `../governance/foundation/boundaries/L1-kernel.md`, `../governance/contracts/protocol/include/session.h`, `../governance/foundation/invariants/I-002-determinism.md`, `../governance/foundation/invariants/I-006-external-effect-boundary.md`
- Targets ADR: `docs/program/adr/adr-007-workspace-isolation.md`, `docs/program/adr/adr-008-connection-lifecycle.md`, `docs/program/adr/adr-009-engine-attachment.md`, `docs/program/adr/adr-010-boot-entrypoint.md`
- Downstream runbook: `docs/archive/legacy/program/milestone-packs/runtime-baselines/workspace/mp-runtime-000-workspaces-lifecycle.md`

## References
- `docs/program/spine.md`
- `docs/program/adr/adr-007-workspace-isolation.md`
- `docs/program/adr/adr-008-connection-lifecycle.md`

# Related Docs
- `docs/program/rfc/README.md`
- Linked ADR and report artifacts
