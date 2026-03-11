# Daemon Architecture Refoundation Baseline (YD-1)

## Goal

Provide an operator/maintainer baseline that matches YD-1 architecture lock
before feature-heavy source-plane slices.

## Canonical Runtime Roles

- `yai`: owner runtime host and canonical truth surface.
- `yai-daemon`: subordinate edge runtime process for distributed acquisition inputs.

## Operational Baseline

1. Build both binaries.
2. Keep owner lifecycle under `yai`.
3. Treat daemon lifecycle as subordinate edge-runtime behavior.
4. Route source-plane operations through owner mediation, not direct truth writes.
5. Treat delegated local action/enforcement as owner-scoped, never sovereign.

## Baseline Commands

```bash
make yai yai-daemon
./build/bin/yai --help
./build/bin/yai-daemon --help
```

## Verification Checklist

- [ ] Help/usage naming is `yai` + `yai-daemon` (no ambiguous aliases).
- [ ] Docs state `distributed acquisition / centralized control`.
- [ ] Docs state daemon is non-authoritative for workspace truth.
- [ ] Docs state daemon is non-authoritative for policy/graph/conflict truth.
- [ ] `exec` is documented as active owner/daemon mediation layer.

## Not Covered Here

- secure peering deployment and overlay bootstrap
- full daemon scan/spool/retry behavior
- multi-peer coordination/conflict handling

Those are separate slices after YD-1.
