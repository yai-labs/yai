---
id: ADR-011
status: draft
effective_date: 2026-02-19
supersedes: []
applies_to:
  runbook: docs/runbooks/root-hardening.md
  phase: 0.1.0
  anchor: "#phase-0-1-0-protocol-guardrails"
law_refs:
  - deps/yai-specs/contracts/invariants/I-001-traceability.md
  - deps/yai-specs/contracts/invariants/I-002-determinism.md
  - deps/yai-specs/contracts/invariants/I-003-governance.md
  - deps/yai-specs/contracts/invariants/I-006-external-effect-boundary.md
  - deps/yai-specs/contracts/boundaries/L1-kernel.md
---
# ADR-011 — Contract Baseline Lock for Milestone 1

## Context

The program needs a stable first milestone before deeper runbook delivery.
Current evidence shows three risks:

- specs/CLI command-surface drift,
- runtime gate success with mandatory-step `SKIP`,
- uneven proof quality for TRL claims.

Without a baseline lock, refactors in specs can outpace consumers and produce false confidence.

## Decision

Milestone 1 enforces a Contract Baseline Lock across `yai-specs`, `yai`, and `yai-cli`.

### Baseline contract scope

- Protocol + runtime headers under `deps/yai-specs/specs/protocol/**`
- CLI command contract under `deps/yai-specs/specs/cli/schema/**`
- Error/authority semantics used for deterministic rejects

### Mandatory controls

1. Specs and CLI behavior must be compared in CI (anti-drift).
2. Required TRL gates must fail when capability is missing (no pass-on-skip for mandatory steps).
3. Verify-core and formal checks must be updated when contract deltas affect invariants/authority/envelope semantics.
4. Cross-repo pins remain explicit and auditable.

## Rationale

This creates a dependable execution base for subsequent runbook phases (root hardening, workspaces, engine attach) and enables evidence-driven TRL progression.

## Consequences

- Positive:
  - Refactors can proceed with controlled blast radius.
  - Evidence quality improves (less ambiguity between "implemented" and "proved").
- Negative:
  - Short-term CI strictness may initially increase failures.
  - Requires coordinated updates across repos for contract-touching changes.

## Law Alignment

- `deps/yai-specs/contracts/invariants/I-001-traceability.md`
- `deps/yai-specs/contracts/invariants/I-002-determinism.md`
- `deps/yai-specs/contracts/invariants/I-003-governance.md`
- `deps/yai-specs/contracts/invariants/I-006-external-effect-boundary.md`
- `deps/yai-specs/contracts/boundaries/L1-kernel.md`

## Status

Draft (to be accepted with Milestone 1 kickoff).
