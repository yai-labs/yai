# Boundary L0 — Vault (Shared Memory Authority)

## Scope
L0 defines the boundary between **Foundation authority** and the **Vault/SHM substrate**.
The Vault is a *mechanism* for state transfer, not a source of truth.

## Foundation Position
- Foundation defines **validity** and **invariants**.
- The Vault is a **carrier** of state, not an arbiter of meaning.

## Non‑Responsibilities
Foundation does NOT define:
- SHM allocation or lifecycle
- Memory layout optimizations
- Performance or paging strategies

## Constraint
If Vault state violates axioms/invariants, the Vault data is **invalid**. The Foundation remains authoritative.
