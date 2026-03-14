---
role: procedural
status: active
audience: operator
owner_domain: runbooks
primary_for: remediation-procedure
depends_on: [docs/architecture/runtime/architecture.md]
---

# Remediation

# Purpose
Contain and recover from runtime, protocol, or governance failures.

# Scope
Covers issue triage, bounded rollback, and recovery verification.

# Relationships
- `lib/runtime/`
- `lib/protocol/`
- `lib/governance/`
- `tools/bin/`

# Canonical Role
Primary remediation runbook.

# Main Body
1. Identify failure surface (runtime/protocol/governance).
2. Apply bounded remediation steps.
3. Re-run qualification gates before closure.

# Related Docs
- `docs/runbooks/operations/operations.md`
- `docs/runbooks/qualification/qualification.md`
