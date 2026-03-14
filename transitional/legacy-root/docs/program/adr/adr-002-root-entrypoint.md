---
role: support
status: historical
audience: governance
owner_domain: program-adr
id: ADR-002
decision_id: ADR-002
supersedes: []
superseded_by: [docs/program/adr/adr-001-single-runtime.md]
implements: []
evidenced_by: [docs/program/reports/runtime-convergence-report.md]
related: []
anchor: "#phase-0-1-1-byte-perfect-router"
applies_to: 
effective_date: 2026-02-18
law_refs: 
phase: 0.1.1
runbook: docs/archive/legacy/program/milestone-packs/runtime-baselines/operations-foundation/mp-runtime-000-root-hardening.md
---
# ADR-002 - Root Control Plane as Canonical Entry Point

> Historical ADR: superseded by single-binary runtime cutover.
> Current authoritative ingress is `~/.yai/run/control.sock` owned by `yai`.

## Context

The runtime needed an explicit public ingress to prevent path drift (`root.sock` vs `control.sock`) and bypass patterns across tooling and operators.

## Decision

Root is the single public ingress policy:

- Canonical socket: `~/.yai/run/root/control.sock`
- Legacy `~/.yai/run/root.sock` is deprecated compatibility only
- Workspace sockets are internal implementation detail

All external clients must enter through Root.

## Rationale

This preserves authority ordering and keeps routing and policy enforcement machine-scoped before requests reach Kernel/Engine.

## Consequences

- Positive:
  - Uniform ingress for CLI, automation, and cockpit.
  - Clear operational contract for pathing and diagnostics.
- Negative:
  - Legacy clients need migration.

## Traceability

- Proposals:
  - `docs/program/rfc/rfc-001-runtime-topology-authority.md`
- Implemented by runbooks:
  - `docs/archive/legacy/program/milestone-packs/runtime-baselines/operations-foundation/mp-runtime-000-root-hardening.md`
- Milestone packs:
  - `docs/archive/legacy/program/milestone-packs/root-hardening/mp-runtime-000-root-hardening-v0-1-5.md`

## Status

Accepted and active.
