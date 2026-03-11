# Mesh Coordination Foundation Baseline (MF-2)

## Objective

Validate that discovered peers become governed coordinated members with
owner-anchored registry state and coordination semantics.

## Baseline checks

1. Verify workspace membership entries exist for attached peers.
2. Verify owner peer registry rows expose role/scope/state/freshness/backlog.
3. Verify workspace coordination summary exposes:
   - peer state rollups
   - coverage/overlap/gap rollups
   - scheduling baseline state
   - replay/ordering/conflict pressure counters
4. Verify peer awareness surfaces are scope-limited and metadata-only.
5. Verify final authority decisions remain owner-side.

## Expected outcomes

- Discovery state is converted into governed coordination membership.
- Coordination state is queryable and graph-consumable.
- Coordination signals assist runtime/operator decisions without sovereignty
  transfer.

## Anti-drift assertions

- Coordinated member != sovereign authority holder.
- Registry awareness != canonical truth ownership.
- Overlap/replay/order signals != final adjudication result.
