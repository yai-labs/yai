# Global-to-Edge Policy Hierarchy Baseline (RF-0.2)

## Goal

Operationalize the hierarchy lock between owner sovereignty and delegated edge
behavior.

## Hierarchy

1. Workspace global policy plane (`yai`, sovereign)
2. Delegated edge policy plane (owner-issued snapshots/grants/capability envelopes)
3. Edge execution/observation plane (`yai-daemon`, subordinate)

## Operator Interpretation

- Edge grants allow bounded local execution, not sovereignty transfer.
- Stale/missing delegated policy must reduce local autonomy.
- Any ambiguity falls back to owner authority and review paths.

## Baseline Verification

```bash
./build/bin/yai --help
./build/bin/yai-daemon --help
```

Expected semantics:

- owner runtime remains canonical truth
- daemon remains subordinate runtime
- delegated local enforcement is owner-scoped
