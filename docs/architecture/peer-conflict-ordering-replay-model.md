# Peer Conflict, Ordering and Replay Baseline (OP-4)

## Purpose

Define the owner-side baseline used when multiple peers emit overlapping or
out-of-order source-plane data into one canonical workspace.

This is a v1.5 discipline layer, not a full conflict engine.

## Baseline Problems Addressed

- same-peer duplicate/replay emits
- cross-peer near-duplicate overlap on asset scope
- late ordering from backlog replay
- backlog flush pressure after reconnection

## Classification Model

Owner classifies each emit outcome into one baseline class:

- `clean`
- `duplicate_replay`
- `overlap_informational`
- `overlap_ambiguous`
- `ordering_late`
- `conflict_requires_review`
- `reject_structural` (already handled as structural reject path)

## Handling Baseline

- `accept`: normal append path (`clean`)
- `accept_with_flag`: replay/late/overlap informational or ambiguous baseline
- `review_stub`: conflict requiring explicit operator review path
- `reject`: only structural validation failures

## Ordering Baseline

Owner uses a practical ordering mix:

- receive time (owner ingest)
- source `observed_at_epoch` when provided
- peer freshness context from owner registry
- idempotency and replay signals

Ordering result is surfaced as:

- `in_order`
- `late`

## Replay Discipline

Replay status is explicit:

- `first_ingest`
- `replay_same_peer`
- `replay_cross_peer`
- `backlog_flush`

Replay is not automatically treated as conflict.

## Persistence Hook

Each emit now persists one `source_ingest_outcome` record with:

- classification and handling action
- ordering/replay/overlap statuses
- accepted counts
- review-required signal
- asset key reference baseline

This keeps conflict/order/replay state queryable and graph-visible.

## Registry and Summary Alignment

Owner peer registry tracks integrity counters per peer:

- replay detected
- overlap detected
- conflict detected
- ordering late
- review required
- last classification and handling action

These roll up into workspace `coordination.integrity` in source query surfaces.

## Query Surfaces

- `yai.workspace.query source` includes integrity counters in coordination summary
- `yai.workspace.query source.conflicts` returns recent `source_ingest_outcome` rows

## Graph Baseline

Graph adds `source_ingest_outcome` node projection and edges:

- `ingest_outcome_for_node`
- `ingest_outcome_for_binding`

This enables topology-level visibility for replay/overlap/conflict signals.

## Limits

Out of scope for OP-4:

- semantic dedup perfection
- automatic merge resolution
- enterprise review workflow engine
- distributed ordering consensus

## References

- `docs/architecture/workspace-peer-orchestration-model.md`
- `docs/architecture/owner-peer-registry-and-coordination-model.md`
- `docs/architecture/workspace-peer-read-model.md`
