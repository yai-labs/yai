---
role: support
status: accepted
audience: governance
owner_domain: program-adr
id: ADR-016
decision_id: ADR-016
supersedes: []
superseded_by: []
implements: []
evidenced_by: [docs/program/reports/runtime-convergence-report.md]
related: []
anchor: "#global-to-edge-policy-hierarchy-lock"
applies_to: 
effective_date: 2026-03-11
phase: RF-0.2
---
# ADR-016 - Global-to-Edge Policy Hierarchy Lock

## Context

After daemon refoundation, edge runtime capabilities must not drift into
implicit sovereign policy behavior.

We need explicit hierarchy and precedence rules before delegated enforcement
and grant lifecycle implementation.

## Decision

YAI locks this hierarchy:

1. workspace global policy plane (owner sovereign, `yai`)
2. delegated edge policy plane (owner-issued snapshots/grants/capability envelopes)
3. edge execution/observation plane (`yai-daemon`, subordinate runtime)

## Authority Rules

- Policy/graph/conflict/canonical DB truth remain owner-side only.
- Edge delegated state is bounded, revocable, and non-sovereign.
- Edge-side behavior cannot exceed/contradict owner-issued scope.

## Precedence Rules

- Global policy overrides delegated edge state.
- Missing/stale delegated state reduces edge autonomy.
- Ambiguity/conflict resolves in favor of owner authority plane.

## Consequences

### Positive

- prevents policy sovereignty drift to edge
- provides stable baseline for grant expiry/refresh/revoke
- aligns runtime, governance and qualification semantics

### Negative

- disconnected edge behavior must degrade conservatively
- full delegated enforcement remains implementation work

## Non-goals

- full policy distribution implementation
- full delegated enforcement implementation
- edge sovereign conflict resolution

## References

- `docs/architecture/global-to-edge-policy-hierarchy-model.md`
- `docs/architecture/source-plane-model-refoundation-rf01.md`
