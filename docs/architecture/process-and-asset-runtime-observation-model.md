# Process and Asset Runtime Observation Model (RF-0.4)

Status: active  
Owner: runtime  
Effective date: 2026-03-11

## Purpose

Define canonical edge observation semantics for subordinate edge runtime.

The source plane is not only asset collection. It is governed edge runtime
observation over assets, processes and node operational state.

## Observable Classes

### Asset observables

- files
- directories
- datasets
- storage-attached resources
- local source records/endpoints

### Process observables

- processes
- jobs/workers
- task runtime chains
- scheduled executions
- local pipeline steps

### Runtime observables

- daemon health and freshness
- spool depth and retry pressure
- connectivity/degraded state
- policy snapshot age
- grant validity state
- action mediation readiness
- integration point reachability

## Scope Separation (non-negotiable)

- observation scope != mediation scope
- mediation scope != enforcement scope

Observing an object never implies local action authority on that object.

## Runtime Signals Are First-Class

Operational node state is part of canonical source-plane semantics, not
secondary telemetry.

Owner-side truth can be grounded on:

- observed data/events
- runtime/process state signals
- degradation and readiness signals
- local outcome/evidence context

## Owner Handoff Contract

Edge runtime emits to owner:

- observed events
- evidence candidates
- process/runtime state signals
- local outcomes and anomaly/degradation context

Canonical acceptance/truth remains owner-side.

## References

- `docs/architecture/source-plane-model.md`
- `docs/architecture/daemon-local-runtime-model.md`
- `docs/architecture/delegated-edge-enforcement-model.md`
- `docs/program/22-adr/ADR-018-process-and-asset-runtime-observation-model.md`
