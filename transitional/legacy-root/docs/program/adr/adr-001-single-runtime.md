---
role: support
status: accepted
audience: governance
owner_domain: program-adr
id: ADR-001
decision_id: ADR-001
supersedes: []
superseded_by: []
implements: [docs/program/rfc/rfc-001-runtime-topology-authority.md]
evidenced_by: [docs/program/reports/runtime-convergence-report.md]
related: []
phase: 0.1.0
runbook: docs/archive/legacy/program/milestone-packs/runtime-baselines/operations-foundation/mp-runtime-000-root-hardening.md
---
# ADR-001 - Single Runtime Per Machine

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

YAI is evolving from a mixed execution model toward a machine-level runtime. The previous shape allowed multiple implicit entry paths and per-workspace execution assumptions that made governance and evidence weaker.

## Decision

YAI adopts one canonical machine runtime composed of:

- Root control plane
- Kernel (L1 authority plane)
- Engine (L2 execution plane)

Workspaces are logical tenants managed by this runtime, not independent daemon stacks.

Distributed source acquisition is allowed only as an edge feed into this owner
runtime model (see `ADR-013`). It does not introduce additional owner runtimes.

## Rationale

A single runtime reduces authority ambiguity, improves observability, and strengthens deterministic enforcement of workspace boundaries.

## Consequences

- Positive:
  - One authoritative ingress and lifecycle model.
  - Better cross-workspace governance and auditable routing.
- Negative:
  - Legacy assumptions around direct workspace access must be removed.
  - Migration work is required in boot/routing and operational docs.

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
