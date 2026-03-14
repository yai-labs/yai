---
role: reference
status: active
audience: developer
owner_domain: reference
depends_on: [docs/reference/protocol/README.md]
---

# Message Types

# Purpose
Define message type families and identifier semantics.

# Scope
Covers protocol message classes and canonical type boundaries.

# Relationships
- `surface.md`
- `rpc.md`

# Canonical Role
Lookup reference for message typing.

# Main Body
Message typing is defined by protocol contract headers (`include/yai/protocol/message_types.h` and contract equivalents). IDs and envelope families are consumed by transport and RPC layers.

# Related Docs
- `docs/reference/protocol/rpc.md`
