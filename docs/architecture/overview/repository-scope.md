---
role: canonical
status: active
audience: architect
owner_domain: architecture
---

# yai repository scope

# Purpose
Defines canonical architecture semantics for the domain.

# Scope
Covers boundaries, responsibilities, and integration semantics for this domain section.

# Relationships
- Parent section README
- Adjacent architecture source documents

# Canonical Role
Authoritative architecture source for its scope.

# Main Body
## Role

`yai` is the runtime implementation repository and governance consumer.

## In scope

- runtime ingress, lifecycle, dispatch, and enforcement realization
- internal unified runtime composition (`core`, `exec`, `data`, `graph`, `knowledge`)
- workspace-first runtime binding and active workspace capability attachment
- runtime consumption/integration of canonical governance surfaces
- owner-runtime side of distributed acquisition plane (`yai` as centralized control plane)

## Out of scope (this tranche)

- canonical governance authorship (owned by `governance`)
- ops official qualification/collateral publishing (owned by `ops`)
- external consumer API and operator UX ownership (owned by `sdk` and `cli`)

## Dependency boundary

`ops` is never a normative source for runtime behavior.
Normative authority comes from `governance`.

# Related Docs
- `docs/architecture/README.md`
- Domain-adjacent architecture documents
