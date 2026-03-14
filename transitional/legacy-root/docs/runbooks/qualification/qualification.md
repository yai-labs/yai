---
role: procedural
status: active
audience: operator
owner_domain: runbooks
primary_for: qualification-procedure
depends_on: [docs/architecture/runtime/architecture.md]
---

# Qualification

# Purpose
Run qualification checks and assert pass/fail outcomes.

# Scope
Defines test/gate sequence and required evidence.

# Relationships
- `tests/integration/`
- `tools/validate/`
- `tools/release/unified_repo_convergence_smoke.sh`
- `docs/reference/README.md`

# Canonical Role
Primary qualification runbook.

# Main Body
1. Execute required validators.
2. Run smoke convergence gate.
3. Record PASS/FAIL with evidence links.

# Related Docs
- `docs/runbooks/operations/operations.md`
- `docs/program/reports/audit-convergence-report.md`
