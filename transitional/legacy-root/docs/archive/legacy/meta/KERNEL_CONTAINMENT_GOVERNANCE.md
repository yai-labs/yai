# K-6 Kernel Containment Governance

Containment is kernel-owned.

## Kernel-owned containment primitives

- containment mode: `none`, `soft`, `scoped`, `contained`, `hardened`
- containment state: `requested`, `active`, `degraded`, `breached`, `suspended`, `revoked`
- escape policy class: `none`, `controlled-admin`, `recovery`, `debug`
- rootfs projection primitive
- namespace linkage primitive
- resource isolation linkage primitive
- breach signaling primitive

## Separation of concerns

- governance decisions are in `kernel/security/containment.c`
- technical primitives are in `kernel/container/{namespaces,rootfs,mounts,limits}.c`
- higher layers may request/consume, but cannot mutate canonical containment state directly

## Kernel ownership lock

Higher layers must not bypass kernel containment roots for:
- container containment activation
- containment state transitions
- containment breach marking
- escape authorization

## Migration discipline

Only low-level salvageable runtime/platform containment pieces are candidates.
Workspace semantics and high-level orchestration logic are excluded.
