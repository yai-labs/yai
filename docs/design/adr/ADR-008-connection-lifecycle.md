---
id: ADR-008
status: accepted
effective_date: 2026-02-18
supersedes: []
applies_to:
  runbook: docs/runbooks/workspaces-lifecycle.md
  phase: 0.1.1
  anchor: "#phase-0-1-1-ws-create-guardrails"
---
# ADR-008 — Connection Lifecycle Semantics

## Context

Connection lifecycle must remain deterministic and auditable across Root and workspace-attached sessions.
Current implementation is partially proven: baseline handshake/lifecycle exists, but full command-surface parity and non-skip gate proof are still incomplete.

## Decision

Connections are one of:

- root session
- workspace-attached session

Handshake establishes protocol validity.
Attach establishes workspace context.

## Rules

- No execution before handshake.
- No runtime-bound execution before explicit attach/workspace context.
- Reconnect requires re-handshake.
- Rejects must be deterministic and trace-correlatable.

## Rationale

Lifecycle is the control boundary between contract validity and execution authority.
Ambiguous status wording in docs can overstate readiness; Milestone 1 needs status language aligned to evidence.

## Law Alignment

- `deps/yai-specs/contracts/invariants/I-001-traceability.md`
- `deps/yai-specs/contracts/invariants/I-002-determinism.md`
- `deps/yai-specs/contracts/invariants/I-003-governance.md`
- `deps/yai-specs/specs/protocol/include/session.h`
- `deps/yai-specs/specs/protocol/include/transport.h`

## Status

Active with partial proof coverage:

- handshake and basic lifecycle semantics are implemented in core paths
- persistent cockpit session model remains pending
- full non-skip gate evidence for lifecycle claims is still required for higher TRL assertions
