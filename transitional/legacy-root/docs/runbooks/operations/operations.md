---
role: procedural
status: active
audience: operator
owner_domain: runbooks
primary_for: operations-procedure
depends_on: [docs/architecture/runtime/architecture.md]
---

# Operations

# Purpose
Execute recurring runtime operations with deterministic checks.

# Scope
Covers preflight, runtime status, and evidence capture.

# Relationships
- `cmd/yai/main.c`
- `lib/runtime/`
- `tools/bin/`
- `tests/integration/`

# Canonical Role
Primary operations runbook.

# Main Body
1. Run core status/health commands.
2. Validate runtime/governance compatibility gates.
3. Capture evidence artifacts for report linkage.

# Related Docs
- `docs/runbooks/qualification/qualification.md`
- `docs/program/reports/runtime-convergence-report.md`
