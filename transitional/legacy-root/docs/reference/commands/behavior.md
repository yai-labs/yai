---
role: reference
status: active
audience: developer
owner_domain: reference
depends_on: [docs/reference/commands/README.md]
---

# CLI Behavior

# Purpose
Define stable behavioral rules for CLI invocation and governance-first command paths.

# Scope
Covers compatibility behavior, boundary rules, and governance-oriented command expectations.

# Relationships
- `docs/reference/commands/surface.md`
- `docs/reference/commands/taxonomy.md`

# Canonical Role
Lookup reference for CLI behavior contracts.

# Main Body
CLI is a client surface. It cannot redefine authority semantics or bypass protocol/runtime governance boundaries. Legacy command aliases are compatibility shims, not ontology owners.

# Related Docs
- `docs/reference/protocol/rpc.md`
