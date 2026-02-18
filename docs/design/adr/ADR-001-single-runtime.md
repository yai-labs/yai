---
id: ADR-001
status: accepted
effective_date: 2026-02-18
supersedes: []
applies_to:
  runbook: docs/runbooks/root-hardening.md
  phase: 0.1.0
  anchor: "#phase-0-1-0-protocol-guardrails"
---
# ADR-001 — Single Runtime Per Machine (Canonical)

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

YAI runs as **one machine-level runtime**, composed of:

- Root Control Plane
- Kernel (L1)
- Engine (L2)

This runtime manages multiple workspaces concurrently.

### Implications

- No per-workspace daemon model long-term.
- No direct CLI-to-workspace socket access.
- The runtime is machine-scoped, not workspace-scoped.

### Constraints

- All runtime-bound requests MUST carry `ws_id`.
- Kernel MUST enforce isolation between workspaces.
- Engine MUST execute effects only under Kernel authority.
- Cross-workspace state sharing is forbidden by default.

### Law Alignment

- A-002 Authority
- I-006 External Effect Boundary
- L1/L2 boundary enforcement

### Status

Architecture locked.
Implementation staged (Root stub active, full multi-tenant pending).

---

## Status

Active
