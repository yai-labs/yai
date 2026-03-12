# Owner Remote Peer Ingress Baseline (MT-2)

## Objective

Validate that remote peer traffic reaches an owner-side governed ingress
boundary and is not implicitly accepted by connectivity alone.

## Baseline checks

1. Verify reachable remote peer can still be ingress-rejected.
2. Verify ingress can accept only limited contribution classes.
3. Verify stale/expired/revoked delegated context constrains ingress acceptance.
4. Verify replay/spool-delayed deliveries can be accepted in restricted mode.
5. Verify ingress acceptance does not bypass owner canonicalization.

## Expected outcomes

- Remote ingress state is explicit (`ready|degraded|restricted|unavailable` baseline).
- Acceptance decisions include scope/validity/legitimacy reasons.
- Owner authority and canonical truth boundaries remain intact.

## Anti-drift assertions

- Reachable peer != accepted peer.
- Ingress accepted != canonicalized truth.
- Connected endpoint != unrestricted contribution rights.
