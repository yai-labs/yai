---
role: canonical
status: active
audience: architect
owner_domain: architecture
---

# Architecture

# Purpose
Defines authoritative architecture semantics for the platform.

# Scope
Covers system topology, boundaries, interactions, and runtime model authority.

# Relationships
- `docs/README.md`
- `docs/runbooks/README.md`
- `docs/reference/README.md`

# Canonical Role
Primary source-of-truth for architecture.

# Main Body
Architecture remains rich but compressed to canonical spines and necessary satellites.

## Scope
Authoritative architecture surface for system topology, boundaries, interactions, and constraints.

## What Belongs Here
- Canonical architecture source documents.
- Required section-level entry docs and minimal supporting satellites.

## What Does Not Belong Here
- Program reporting history.
- Migration/closeout/refoundation notes.

## Navigation Order
1. `overview/`
2. `runtime/`
3. `workspace/`
4. `distributed-runtime/`
5. `governance/`
6. `protocol/`
7. `data-runtime/`
8. `intelligence-runtime/`
9. `system-theory/`

## Extension Rules
- One canonical owner doc per architecture domain.
- New architecture docs require non-overlapping scope with existing canonical docs.

# Related Docs
- `overview/repository-scope.md`
- `runtime/runtime-architecture.md`
- `workspace/workspace-architecture.md`
