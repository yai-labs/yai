# ER-2 Edge State, Spool, Retry, Health Baseline

Status: active  
Owner: runtime  
Effective date: 2026-03-11

## Purpose

Define the operational baseline for subordinate edge runtime continuity and
resilience.

## Local Durability Contract

- `spool/queue` stores not-yet-accepted units for owner delivery.
- `spool/delivered` stores locally delivered confirmations.
- `spool/failed` stores retry-exhausted terminal units.
- spool state is operational edge durability, never owner canonical truth.

## Retry Contract (v1 baseline)

- bounded exponential backoff for transient failures;
- retry-due units remain queued;
- terminal demotion after bounded attempts;
- reconnect can resume redelivery without data-loss assumptions.

## Health/Freshness Contract

`state/health.v1.json` and `state/edge-operational-state.v1.json` expose:

- health state (`ready|degraded|disconnected|stopping`)
- connectivity state
- freshness state
- spool/retry pressure
- policy/grant staleness placeholders
- degradation reason

## Sovereignty Guardrail

Local continuity is allowed; local sovereignty is not.
Owner-side acceptance still defines canonical truth for policy/graph/conflict
and final state materialization.

