# Protocol Runtime Surface

`contracts/protocol/runtime/` contains runtime-scoped protocol helper contracts.

## Canonical artifact

- `include/rpc_runtime.h`

## Interpretation

This header is a shared protocol-runtime helper surface.
It is not tied to legacy package identities.
It supports envelope validation/response preparation for runtime modules in `core`, `exec`, and `brain`.
