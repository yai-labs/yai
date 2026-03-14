---
role: support
status: accepted
audience: governance
owner_domain: program-adr
id: ADR-006
decision_id: ADR-006
supersedes: []
superseded_by: []
implements: [docs/program/rfc/rfc-002-unified-rpc-cli-contract.md]
evidenced_by: [docs/program/reports/runtime-convergence-report.md]
related: []
phase: 0.1.0
runbook: docs/archive/legacy/program/milestone-packs/runtime-baselines/operations-foundation/mp-runtime-000-root-hardening.md
---
# ADR-006 - Strict Unified RPC Contract

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

Milestone 1 required one explicit envelope and command baseline across specs, core runtime, and CLI. Without this, CI could be green while operational behavior drifted.

## Decision

All communication follows the pinned binary contract in `../governance`, with command semantics anchored by CLI contract artifacts.

Mandatory rules:

- Handshake before effectful commands
- Workspace context for runtime-bound commands
- Role + arming enforcement for privileged operations
- Deterministic reject semantics mapped to contract identifiers
- No mandatory proof claims on skipped gates

## Rationale

A strict contract baseline makes drift observable and converts gate output into reliable evidence.

## Consequences

- Positive:
  - Better parity between foundation/spec and implementation.
  - Stronger auditability for TRL progression.
- Negative:
  - Tighter CI can increase short-term failures during migration.

## Traceability

- Proposals:
  - `docs/program/rfc/rfc-002-unified-rpc-cli-contract.md`
  - `docs/program/rfc/rfc-005-formal-coverage-roadmap.md`
- Implemented by runbooks:
  - `docs/archive/legacy/program/milestone-packs/runtime-baselines/operations-foundation/mp-runtime-000-root-hardening.md`
- Milestone packs:
  - `docs/archive/legacy/program/milestone-packs/root-hardening/mp-runtime-000-root-hardening-v0-1-5.md`
  - `docs/archive/legacy/program/milestone-packs/root-hardening/mp-runtime-000-root-hardening-v0-1-5.md`

## Status

Accepted and active.

# Related Docs
- `docs/program/adr/README.md`
- Linked RFC/report artifacts
