# Kernel Migration Map (K-2)

Canonical source of truth: `kernel/PRIMITIVE_OWNERSHIP_MATRIX.md`.

This file is intentionally short and tracks execution status only.

## Sources

- `include/yai/runtime/*`, `lib/runtime/*`
- `include/yai/platform/*`, `lib/platform/*`
- `include/yai/protocol/*`, `lib/protocol/*`

## Migration gates

- move only modules classified as `KERNEL_OWNER`
- split mixed modules before migration
- route non-kernel modules to `sys/*` or mark `LEGACY_DEMOLISH`

## Hard constraints

- no workspace-centric semantics in kernel path
- no orchestration/daemon/provider/graph/data engines in kernel path
- `runtime/` is migration source, not future center
