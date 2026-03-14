---
role: reference
status: active
audience: developer
owner_domain: reference
primary_for: protocol-reference
depends_on: [docs/architecture/protocol/transport.md]
---
# Protocol Reference

# Purpose
Provide protocol lookup reference aligned to protocol headers and runtime implementation.

## Scope
Protocol surface, message types, transport, RPC, and binary references.

## What Belongs Here
- Contract lookup docs tied to `include/yai/protocol/**` and `lib/protocol/**`.

## What Does Not Belong Here
- Governance or delivery reports.
- Historical transition notes.

## Navigation Order
1. `surface.md`
2. `message-types.md`
3. `transport.md`
4. `rpc.md`
5. `binary.md`

## Extension Rules
- Add files only for new protocol contract families.
- Do not duplicate these canonical core entries.

# Relationships
- `include/yai/protocol/`
- `lib/protocol/`
- `include/yai/protocol/rpc/`
- `include/yai/protocol/binary/`
- `include/yai/protocol/transport/`

# Canonical Role
Primary protocol reference entrypoint.

# Main Body
Use child docs for direct contract lookup.

# Related Docs
- `docs/architecture/protocol/README.md`
- `docs/reference/commands/README.md`
