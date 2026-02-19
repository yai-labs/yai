---
id: ADR-005
status: accepted
effective_date: 2026-02-18
supersedes: []
applies_to:
  runbook: docs/runbooks/mind-redis-stm.md
  phase: v5.3
  anchor: "#phase-mind-proposer"
law_refs:
  - deps/yai-specs/contracts/invariants/I-002-determinism.md
  - deps/yai-specs/contracts/invariants/I-004-cognitive-reconfiguration.md
  - deps/yai-specs/contracts/invariants/I-006-external-effect-boundary.md
  - deps/yai-specs/contracts/boundaries/L3-mind.md
---
# ADR-005 - Mind as Workspace-Scoped Proposer (L3)

## Context

Cognitive features need to evolve without becoming a hidden execution authority or a cross-workspace leakage vector.

## Decision

Each workspace owns its Mind context. Mind is proposal-oriented and non-authoritative.

Mind may:

- Build internal graph/cognitive state
- Generate plans and proposals

Mind may not:

- Execute external effects directly
- Bypass Engine/Kernel governance
- Access other workspace state by default

## Rationale

This model preserves cognitive isolation while keeping effectful operations under governed L1/L2 control.

## Consequences

- Positive:
  - Safe evolution of cognition features.
  - Clear boundary between reasoning and execution.
- Negative:
  - Additional orchestration needed for proposal-to-execution flows.

## Status

Accepted and active.
