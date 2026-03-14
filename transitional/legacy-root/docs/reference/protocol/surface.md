---
role: reference
status: active
audience: developer
owner_domain: reference
depends_on: [docs/reference/protocol/README.md]
---

# Protocol Surface

# Purpose
Describe the public protocol surface exposed by headers and runtime implementation.

# Scope
Maps reference surface to `include/yai/protocol/**` and `lib/protocol/**`.

# Relationships
- `message-types.md`
- `transport.md`
- `rpc.md`
- `binary.md`

# Canonical Role
Lookup reference for protocol surface boundaries.

# Main Body
Canonical protocol headers include `message_types.h`, `transport_contract.h`, `rpc_runtime.h`, and source-plane contract headers. Runtime implementation resides under `lib/protocol/`.

# Related Docs
- `docs/architecture/protocol/transport.md`
