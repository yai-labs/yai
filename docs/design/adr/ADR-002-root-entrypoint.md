---
id: ADR-002
status: accepted
effective_date: 2026-02-18
supersedes: []
applies_to:
  runbook: docs/runbooks/root-hardening.md
  phase: 0.1.1
  anchor: "#phase-0-1-1-byte-perfect-router"
---
# ADR-002 — Root Control Plane as Canonical Entry Point

## Context

# YAI Architecture Decisions (Law-Aligned, 2026 Revision)

This document captures the **machine-level architecture commitments**
of YAI as of the current runtime refactor phase.

It is grounded in `deps/yai-specs/contracts/` invariants and reflects the
post-envelope, post-authority enforcement state.

The architecture is stratified across:

- L0 — Vault (immutable identity & ABI boundary)
- L1 — Kernel (authority, sessions, isolation)
- L2 — Engine (execution gates)
- L3 — Mind (proposal-only cognition per workspace)
- Root — Machine Control Plane (runtime governor)

---

### Decision

Introduce a **Root Control Plane** as the only public entrypoint
for CLI and cockpit.

All external clients MUST connect to:

    ~/.yai/run/root.sock

Workspace sockets are internal-only.

### Responsibilities (Root)

- Runtime health & status
- Workspace registry
- Workspace spawn/attach/detach
- Machine-level boundary enforcement
- Routing to workspace plane

### Non-Goals

- Root does NOT execute engine gates
- Root does NOT host Mind logic
- Root does NOT mutate workspace memory

Root is a governor, not an executor.

### Rationale

Prevents:

- Direct CLI → workspace bypass
- Unauthorized multi-tenant conflicts
- Daemon explosion per workspace

### Status

Stub implementation active.
Routing layer under integration.

---

## Status

Active
