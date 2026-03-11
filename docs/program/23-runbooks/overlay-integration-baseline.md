# Overlay Integration Baseline (MT-3)

## Objective

Validate overlay-native distributed deployment behavior for owner and peers
without authority drift.

## Baseline checks

1. Verify peer can move outside LAN and remain reachable via private overlay.
2. Verify node identity continuity across endpoint/path mutation.
3. Verify owner ingress restrictions still apply on overlay paths.
4. Verify temporary overlay loss degrades transport while local edge runtime can
   continue in constrained mode.
5. Verify refresh/revalidation resumes after overlay recovery.

## Expected outcomes

- Overlay-aware targeting and path state are inspectable.
- Endpoint mutation/rebind events are visible as transport/runtime state.
- Reachability transitions do not alter enrollment/trust/sovereignty boundaries.

## Anti-drift assertions

- Overlay present != trusted/enrolled peer.
- Path restored != unrestricted delegated scope.
- Transport identity continuity != canonical authority transfer.
