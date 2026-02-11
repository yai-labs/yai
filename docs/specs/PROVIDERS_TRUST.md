# Providers Trust Lifecycle (R4)

## Purpose
Define a deterministic, persisted and auditable lifecycle for providers.

## Canonical Stores
- Global trust store: `~/.yai/trust/providers.json`
- Workspace active attachment: `~/.yai/run/<ws>/providers.json`

## States
- `discovered`
- `paired`
- `attached`
- `detached`
- `revoked`

## Allowed Transitions
- `discovered -> paired`
- `paired -> attached`
- `attached -> detached`
- `detached -> attached`
- `* -> revoked`

## Hard Rules
- `attached` is forbidden unless provider is at least `paired`.
- `revoked` blocks new `attach`.
- Every valid transition must emit an audit event with:
  - `provider_id`
  - `previous_state`
  - `new_state`
  - `trust_snapshot_hash`

## Events
- `provider_discovered`
- `provider_paired`
- `provider_attached`
- `provider_detached`
- `provider_revoked`

## Replay Safety
- Trust store is a materialized state; events remain the source of truth.
- Live discovery is environment-dependent and not required for deterministic replay.
