# contracts/

`contracts/` is the canonical operational binding layer of governance.

## Scope

- `control/`: control-plane call/reply and authority-facing contracts.
- `protocol/`: protocol headers and runtime RPC contract surfaces.
- `vault/`: vault ABI contract schema and generated headers.
- `providers/`: provider trust and attachment contracts.
- `compliance/`: compliance-facing binding notes/surfaces.
- `cli/`: CLI-facing contractual interface notes.

## Role

Use `contracts/` for operational compatibility and binding semantics between
runtime, protocol, tooling, and governance surfaces.

Use `../schema/` for structural object schemas.

Contracts are normative and must stay aligned with:

- `../manifests/` publish/runtime contract spine
- `../registry/` canonical machine vocabulary
- `../grammar/` semantic grammar and schema sets
- `../schema/` structural object validation layer
