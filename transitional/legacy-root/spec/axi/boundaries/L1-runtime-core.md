# L1 — Kernel Privileged Core Boundary

## Boundary role

L1 is the privileged kernel boundary, owned by `kernel/`.
It is the hard point where kernel-grade authority becomes enforceable for:
- session admission
- containment
- grants/capability checks
- lifecycle and registry roots

## What L1 owns

- ABI root, handles, error/type base
- kernel lifecycle state machine and readiness gating
- privileged session admission state
- containment root state and escape/breach primitives
- grants root validity/checks and revocation hooks
- kernel policy hooks for low-level admission operations
- privileged registry roots for kernel objects
- low-level container primitives in `kernel/container/*` (namespaces, mounts, limits, rootfs)

## What L1 does not own

- orchestration/workflow logic
- governance and high-level policy composition
- graph/data service logic
- high-level network coordination
- daemon manager business flows
- operator-facing UX/SDK/CLI surfaces

## Transitional clarity

`runtime/compatibility/` is migration-only and never a replacement for L1 ownership.

Container domain runtime management is owned by `sys/container/*`, not by L1.
