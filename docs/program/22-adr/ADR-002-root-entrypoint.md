---
id: ADR-002
status: historical
effective_date: 2026-02-18
supersedes: []
applies_to:
  runbook: docs/program/23-runbooks/root-hardening.md
  phase: 0.1.1
  anchor: "#phase-0-1-1-byte-perfect-router"
law_refs:
  - deps/law/foundation/axioms/A-002-authority.md
  - deps/law/foundation/invariants/I-003-governance.md
  - deps/law/foundation/boundaries/L1-kernel.md
  - deps/law/contracts/protocol/include/transport.h
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
  - `docs/program/21-rfc/RFC-001-runtime-topology-and-authority.md`
- Implemented by runbooks:
  - `docs/program/23-runbooks/root-hardening.md`
- Milestone packs:
  - `docs/program/24-milestone-packs/root-hardening/MP-ROOT-HARDENING-0.1.0.md`

## Status

Accepted and active.
