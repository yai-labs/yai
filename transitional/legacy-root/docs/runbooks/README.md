---
role: procedural
status: active
audience: operator
owner_domain: runbooks
depends_on: [docs/README.md,docs/architecture/overview/system-overview.md]
---
# Runbooks

# Purpose
Define executable procedures for operating and validating the platform.

## Scope
Operations, qualification, demos, and remediation procedures.

## What Belongs Here
- Ordered procedures with prerequisites, commands, expected outputs, and escalation paths.

## What Does Not Belong Here
- Architecture model authority.
- Program reporting history.
- Migration notes.

## Navigation Order
1. `operations/`
2. `qualification/`
3. `demos/`
4. `remediation/`

## Extension Rules
- Add runbooks only for distinct executable procedures.
- Keep scenario variants as appendices/satellites.

# Relationships
- `cmd/`
- `tools/bin/`
- `tools/release/`
- `tests/`
- `docs/architecture/README.md`

# Canonical Role
Procedural authority for operational execution.

# Main Body
Runbooks are execution-first and evidence-oriented.

# Related Docs
- `operations/README.md`
- `qualification/README.md`
- `demos/README.md`
- `remediation/README.md`
