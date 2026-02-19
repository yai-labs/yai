---
id: ADR-002
status: accepted
effective_date: 2026-02-18
supersedes: []
applies_to:
  runbook: docs/runbooks/root-hardening.md
  phase: 0.1.1
  anchor: "#phase-0-1-1-byte-perfect-router"
law_refs:
  - deps/yai-specs/contracts/axioms/A-002-authority.md
  - deps/yai-specs/contracts/invariants/I-003-governance.md
  - deps/yai-specs/contracts/boundaries/L1-kernel.md
  - deps/yai-specs/specs/protocol/include/transport.h
---
# ADR-002 - Root Control Plane as Canonical Entry Point

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

## Status

Accepted and active.
