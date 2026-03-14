---
role: support
status: active
audience: governance
owner_domain: program
decision_id: PROGRAM-INDEX
depends_on: [docs/README.md,docs/architecture/overview/system-overview.md,docs/runbooks/README.md]
---
# Program

# Purpose
Provide governance-of-work artifacts without competing with platform architecture/reference authority.

## Scope
ADR, RFC, program policies, templates, canonical reports, and program archive.

## What Belongs Here
- `adr/`
- `rfc/`
- `policies/`
- `reports/`
- `templates/`
- `archive/`

## What Does Not Belong Here
- Architecture authority docs.
- Live milestone stream trees.
- Delivery-phase progress notes in live root.

## Navigation Order
1. `adr/`
2. `rfc/`
3. `policies/`
4. `reports/`
5. `templates/`
6. `archive/`

## Extension Rules
- Program policy docs stay program-specific.
- Docs-global policies stay under `docs/policies/`.
- Historical delivery artifacts go under `docs/program/archive/**`.

# Relationships
- `docs/architecture/README.md`
- `docs/runbooks/README.md`
- `tools/bin/yai-governance-compat-check`
- `tools/release/unified_repo_convergence_smoke.sh`

# Canonical Role
Secondary governance surface for decision lifecycle and evidence.

# Main Body
Use ADR/RFC for decisions and reports for compact evidence.

# Related Docs
- `policies/README.md`
- `reports/README.md`
- `archive/README.md`
