# RF-0.4 Process and Asset Runtime Observation Baseline

Status: active  
Owner: runtime  
Effective date: 2026-03-11

## Purpose

Define baseline operational interpretation of edge observation under the
subordinate runtime model.

## Observation Classes (v1 baseline)

1. Asset observables
- files/directories/datasets/storage-attached resources

2. Process observables
- processes/jobs/workers/pipeline steps/scheduled executions

3. Runtime observables
- freshness/health/spool depth/retry pressure/connectivity
- policy snapshot age/grant validity/action readiness

## Non-Negotiable Scope Rule

- Observation scope does not imply mediation scope.
- Mediation scope does not imply enforcement scope.
- Local action remains delegated, owner-scoped, and revocable.

## Owner Handoff Baseline

Edge runtime sends owner:

- observed events
- evidence candidates
- process/runtime state signals
- local degradation/anomaly context

Owner remains canonical authority for policy/graph/conflict/state truth.

## Operator Reading Baseline

When reviewing source-plane status, interpret runtime signals as first-class
inputs for owner decisions (not as mere telemetry).

If policy/grant state is stale, local autonomy must reduce (observe-only or
escalate), never expand.

## References

- `docs/architecture/process-and-asset-runtime-observation-model.md`
- `docs/program/adr/ADR-018-process-and-asset-runtime-observation-model.md`
- `docs/architecture/delegated-edge-enforcement-model.md`
