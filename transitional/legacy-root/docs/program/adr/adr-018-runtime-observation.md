---
role: support
status: accepted
audience: governance
owner_domain: program-adr
id: ADR-018
decision_id: ADR-018
supersedes: []
superseded_by: []
implements: []
evidenced_by: [docs/program/reports/runtime-convergence-report.md]
related: []
anchor: "#process-and-asset-runtime-observation-model"
applies_to: 
effective_date: 2026-03-11
phase: RF-0.4
---
# ADR-018 - Process and Asset Runtime Observation Model

## Context

Without explicit observation semantics, edge runtime can be mis-modeled as a
file collector only, weakening future enforcement/coordination/qualification.

## Decision

YAI locks edge observation model across three classes:

- asset observables
- process observables
- runtime observables

## Scope Rule

Observation scope does not imply mediation or enforcement scope.

## Owner Truth Rule

Edge observation outputs (including runtime/process state) are upstream inputs.
Canonical policy/graph/conflict/state truth remains owner-side.

## Consequences

### Positive

- realistic edge runtime semantics for qualification and demos
- stronger foundation for process/job/pipeline scenarios
- cleaner grounding inputs for owner-side decisions

### Negative

- requires explicit modeling of runtime signals in later slices
- increases semantic surface for query/graph waves

## Non-goals

- full process instrumentation implementation
- full runtime signal ingestion implementation
- automatic action mediation from observation alone

## References

- `docs/architecture/process-and-asset-runtime-observation-model.md`
- `docs/architecture/delegated-edge-enforcement-model.md`
