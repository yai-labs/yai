---
id: ADR-006
status: accepted
effective_date: 2026-02-18
supersedes: []
applies_to:
  runbook: docs/20-governance/23-runbooks/root-hardening.md
  phase: 0.1.0
  anchor: "#phase-0-1-0-protocol-guardrails"
law_refs:
  - deps/yai-specs/contracts/invariants/I-001-traceability.md
  - deps/yai-specs/contracts/invariants/I-002-determinism.md
  - deps/yai-specs/contracts/invariants/I-003-governance.md
  - deps/yai-specs/contracts/boundaries/L1-kernel.md
  - deps/yai-specs/contracts/boundaries/L2-engine.md
---
# ADR-006 - Strict Unified RPC Contract

## Context

Milestone 1 required one explicit envelope and command baseline across specs, core runtime, and CLI. Without this, CI could be green while operational behavior drifted.

## Decision

All communication follows the pinned binary contract in `deps/yai-specs`, with command semantics anchored by CLI contract artifacts.

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
  - Better parity between law/spec and implementation.
  - Stronger auditability for TRL progression.
- Negative:
  - Tighter CI can increase short-term failures during migration.

## Traceability

- Proposals:
  - `docs/20-governance/21-proposals/PRP-002-unified-rpc-and-cli-contract.md`
  - `docs/20-governance/21-proposals/PRP-005-formal-coverage-roadmap.md`
- Implemented by runbooks:
  - `docs/20-governance/23-runbooks/root-hardening.md`
- Milestone packs:
  - `docs/20-governance/24-milestone-packs/root-hardening/MP-ROOT-HARDENING-0.1.0.md`
  - `docs/20-governance/24-milestone-packs/root-hardening/MP-ROOT-HARDENING-0.1.5.md`

## Status

Accepted and active.
