# Protocol Contracts

Protocol contracts are canonical under:

- `lib/protocol/contracts/schema/` for wire/control/provider/vault schemas.
- `include/yai/protocol/contracts/` for protocol headers and ABI surfaces.

## Scope

- `control/`: control-plane call/reply and authority-facing schemas.
- `protocol/`: protocol headers and runtime RPC contract surfaces.
- `vault/`: vault ABI schema and generated headers.
- `providers/`: provider trust and attachment schemas.
- `compliance/`: compliance-facing binding notes/surfaces.
- `cli/`: CLI-facing contractual interface notes.

## Role

Use protocol contracts for operational compatibility and binding semantics
between runtime, protocol, tooling, and governance surfaces.

Use `governance/schema/` for structural governance object schemas.

Contracts are normative and must stay aligned with:

- `governance/manifests/` publish/runtime contract spine
- `governance/registry/` canonical machine vocabulary
- `governance/grammar/` semantic grammar and schema sets
- `governance/schema/` structural object validation layer
