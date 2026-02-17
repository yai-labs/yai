# ADR-006 — Strict Unified RPC Contract

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

All communication follows a single binary envelope contract:

Envelope fields:

- magic
- version
- ws_id
- trace_id
- command_id
- role
- arming
- payload_len

### Mandatory Rules

- Handshake required
- `ws_id` required for runtime-bound commands
- `arming + role` required for privileged commands
- Deterministic error responses (with ws_id + trace_id)

### Prohibited

- Parallel protocols
- JSON-only side channels
- CLI-specific shortcuts

### Status

Envelope enforcement active.
CLI authority injection active.

---

## Status

Active
