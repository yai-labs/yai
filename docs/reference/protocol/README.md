---
role: reference
status: active
audience: developer
owner_domain: reference
---

# Protocol Reference

# Purpose
Defines protocol-family contract reference for control and data interaction surfaces.

# Scope
Covers protocol families `cli`, `control`, `protocol`, `providers`, `vault`, and `compliance`.

# Relationships
- `docs/reference/README.md`
- `docs/architecture/protocol/README.md`

# Canonical Role
Authoritative protocol-reference index for contract consumers.

# Main Body
Protocol reference is organized by contract family with family-level entrypoints.

## Scope
Canonical protocol-surface reference grouped by contract family.

## What Belongs Here
- Contract and interface reference for `cli`, `control`, `protocol`, `providers`, `vault`, and `compliance`.
- Family-specific lookup docs for protocol consumers.

## What Does Not Belong Here
- Generic architecture narratives.
- Migration overlays and superseded contract notes.
- Procedure/runbook content.

## Navigation Order
1. `cli/`
2. `control/`
3. `protocol/`
4. `providers/`
5. `vault/`
6. `compliance/`

## Extension Rules
- Add new protocol material under the matching family folder.
- Keep family README files as entrypoints.
- Keep `contracts/` transitional-only.

# Related Docs
- `cli/README.md`
- `control/README.md`
- `protocol/README.md`
