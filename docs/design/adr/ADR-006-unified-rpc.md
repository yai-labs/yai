---
id: ADR-006
status: accepted
effective_date: 2026-02-18
supersedes: []
applies_to:
  runbook: docs/runbooks/root-hardening.md
  phase: 0.1.0
  anchor: "#phase-0-1-0-protocol-guardrails"
---
# ADR-006 — Strict Unified RPC Contract

## Context

Milestone 1 requires one explicit contract baseline across specs, core, and CLI.
Without a strict mapping between envelope contract and exposed command surface, CI can report green while runtime proofs remain partial.

## Decision

All communication follows one binary envelope contract anchored in specs:

- `deps/yai-specs/specs/protocol/include/transport.h`
- `deps/yai-specs/specs/protocol/include/protocol.h`
- `deps/yai-specs/specs/protocol/include/yai_protocol_ids.h`
- `deps/yai-specs/specs/protocol/include/errors.h`
- `deps/yai-specs/specs/protocol/include/auth.h`
- `deps/yai-specs/specs/protocol/include/roles.h`
- `deps/yai-specs/specs/protocol/include/session.h`
- `deps/yai-specs/specs/protocol/runtime/include/rpc_runtime.h`

And command semantics are anchored by pinned CLI contract artifacts:

- `deps/yai-specs/specs/cli/schema/commands.v1.json`
- `deps/yai-specs/specs/cli/schema/commands.schema.json`

## Mandatory Rules

- Handshake required before effectful commands.
- `ws_id` required for runtime-bound commands.
- `arming + role` required for privileged commands.
- Deterministic reject semantics with stable code mapping to specs.
- No behavior may be claimed as "proved" if required gates are skipped.

## Prohibited

- Parallel protocol surfaces
- Out-of-contract side channels
- CLI shortcuts that bypass contract semantics
- Green evidence based on mandatory-step `SKIP`

## Rationale

This establishes an auditable baseline for Milestone 1:

- contract and implementation drift are machine-detectable
- runtime gates can be interpreted as real evidence
- TRL progression is tied to non-skipped proof

## Law Alignment

- `deps/yai-specs/contracts/invariants/I-001-traceability.md`
- `deps/yai-specs/contracts/invariants/I-002-determinism.md`
- `deps/yai-specs/contracts/invariants/I-003-governance.md`
- `deps/yai-specs/contracts/boundaries/L1-kernel.md`
- `deps/yai-specs/contracts/boundaries/L2-engine.md`

## Status

Active. Milestone 1 requires CI/gates to enforce this contract baseline explicitly.
