---
role: procedural
status: active
audience: operator
owner_domain: runbooks
primary_for: demos-procedure
depends_on: [docs/architecture/overview/system-overview.md]
---

# Demo

# Purpose
Run deterministic capability demos aligned to current platform architecture.

# Scope
Covers demo preconditions, execution sequence, and expected outputs.

# Relationships
- `cmd/`
- `tests/`
- `docs/architecture/overview/system-overview.md`
- `docs/reference/commands/surface.md`

# Canonical Role
Primary demo execution runbook.

# Main Body
1. Validate build and command surfaces.
2. Execute selected demo scenario.
3. Capture outputs and link to report evidence.

# Related Docs
- `docs/runbooks/qualification/qualification.md`
- `docs/program/reports/runtime-convergence-report.md`
