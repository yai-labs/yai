# Providers Trust Law (v1)

## Authority Boundary
- Provider trust never changes L1 authority.
- Provider trust state is control-plane metadata.
- Kernel remains final authority for capability execution.

## Canonical Paths
- Trust registry: `~/.yai/trust/providers.json`
- Workspace attachment: `~/.yai/run/<ws>/providers.json`

## Lifecycle
- discover: create/update provider entry with `discovered`
- pair: explicit transition to `paired`
- attach: explicit transition to `attached` for one workspace
- detach: transition to `detached`
- revoke: transition to `revoked` (attach denied)

## Invariants
- No implicit trust.
- No attach without pair.
- Every transition must emit an audit event.
- Trust records must be tamper-evident via integrity hash/signature fields.
