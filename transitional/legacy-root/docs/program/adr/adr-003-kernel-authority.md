---
role: support
status: accepted
audience: governance
owner_domain: program-adr
id: ADR-003
decision_id: ADR-003
supersedes: []
superseded_by: []
implements: [docs/program/rfc/rfc-001-runtime-topology-authority.md]
evidenced_by: [docs/program/reports/runtime-convergence-report.md]
related: []
phase: 0.1.2
runbook: docs/archive/legacy/program/milestone-packs/runtime-baselines/operations-foundation/mp-runtime-000-root-hardening.md
---
# ADR-003 - Kernel as Authority Plane (L1)

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
## Context

Authority checks were historically mixed with execution paths. This weakened guarantees around role/arming/workspace enforcement.

## Decision

Kernel is the sole authority plane and validates:

- Handshake and protocol conformance
- Role and arming constraints
- Workspace binding and session ownership

No effectful execution is allowed before Kernel authorization.

## Rationale

Separating authority from execution preserves deterministic policy behavior and ensures that Engine cannot be used as an authorization surface.

## Consequences

- Positive:
  - Clear trust boundary between policy and execution.
  - Consistent reject semantics for unauthorized requests.
- Negative:
  - Additional coordination needed when evolving command lifecycle.

## Traceability

- Proposals:
  - `docs/program/rfc/rfc-001-runtime-topology-authority.md`
- Implemented by runbooks:
  - `docs/archive/legacy/program/milestone-packs/runtime-baselines/operations-foundation/mp-runtime-000-root-hardening.md`
- Milestone packs:
  - `docs/archive/legacy/program/milestone-packs/root-hardening/mp-runtime-000-root-hardening-v0-1-5.md`

## Status

Accepted and active.

# Related Docs
- `docs/program/adr/README.md`
- Linked RFC/report artifacts
