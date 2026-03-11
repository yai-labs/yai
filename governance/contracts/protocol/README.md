# Protocol Contracts

`contracts/protocol/` defines shared wire/runtime protocol contracts.

Protocol is a cross-cutting layer.
It is not an authority domain.

## Scope

- envelope and framing rules
- transport/session/auth/audit contract types
- protocol identifier allocation
- runtime RPC helper surface (`runtime/include/rpc_runtime.h`)

## Ontology alignment

- Protocol contracts are shared across `core`, `exec`, and `brain`.
- Protocol must not encode historical package topology as primary semantics.
- Legacy terms may remain in comments/IDs only when required for compatibility.
