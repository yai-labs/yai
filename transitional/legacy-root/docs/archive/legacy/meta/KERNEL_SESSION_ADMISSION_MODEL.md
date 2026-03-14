# K-5 Kernel Session Admission Model

## Kernel-owned session classes

- `global`
- `container-bound`
- `privileged`
- `service`

## Kernel-owned admission states

- `pending`
- `admitted`
- `bound`
- `suspended`
- `revoked`
- `closed`

## Kernel-owned primitives

- `yai_kernel_session_admit(...)`
- `yai_kernel_session_bind_container(...)`
- `yai_kernel_session_suspend(...)`
- `yai_kernel_session_revoke(...)`
- `yai_kernel_session_close(...)`

## Ownership lock

Higher layers cannot create, bind, suspend, revoke, or close canonical sessions
outside kernel-mediated entrypoints.

Container binding is kernel-mediated and represented in the kernel session registry.
Privilege class is kernel-owned, and revocation is kernel-owned.

## Migration note

Current `runtime/session/*` remains migration source only.
Canonical session semantics now live in `kernel/include/yai/kernel/session.h`
and `kernel/session/*`.
