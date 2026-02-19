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
# ADR-002 — Root Control Plane as Canonical Entry Point

## Context

YAI requires a single machine-level ingress to prevent authority bypass and workspace coupling drift.

Current repository state showed path ambiguity in docs (`root.sock`) versus runtime/gate conventions (`control.sock` under root run dir).
Milestone 1 needs this normalized so contract checks and gates are unambiguous.

## Decision

Introduce and keep a single public Root ingress policy:

- Canonical external client socket: `~/.yai/run/root/control.sock`
- Legacy alias `~/.yai/run/root.sock` is non-canonical and must be treated as deprecated compatibility surface only.
- Workspace sockets remain internal-only.

All external clients (CLI/cockpit/automation) must reach runtime authority through Root.

## Responsibilities (Root)

- Runtime health and status
- Workspace registry and routing
- Machine-level boundary enforcement
- Protocol/session gateway before Kernel/Engine planes

## Non-Goals

- Root does not execute engine gates
- Root does not host mind cognition logic
- Root does not mutate workspace memory directly

Root is a governor/router, not an execution plane.

## Rationale

This prevents:

- direct CLI to workspace bypass
- tenant boundary erosion
- path-level drift that breaks gate reproducibility

## Law Alignment

- `deps/yai-specs/contracts/axioms/A-002-authority.md`
- `deps/yai-specs/contracts/invariants/I-003-governance.md`
- `deps/yai-specs/contracts/boundaries/L1-kernel.md`
- `deps/yai-specs/specs/protocol/include/transport.h`

## Status

Active. Canonical path policy is now explicit and normalized for Milestone 1.
