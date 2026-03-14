---
role: canonical
status: active
audience: architect
owner_domain: architecture
primary_for: runtime-architecture
---

# Runtime Architecture

# Purpose
Define the canonical runtime authority model and execution boundaries.

# Scope
Covers runtime ownership, enforcement flow, resolution flow, and runtime data sinks.

# Relationships
- `include/yai/runtime/`
- `lib/runtime/`
- `lib/governance/`
- `lib/orchestration/`
- `cmd/yai/main.c`

# Canonical Role
Authoritative architecture source for runtime semantics.

# Main Body
The `yai` process is the owner runtime authority. Edge runtimes are subordinate and never become canonical truth owners.

## Canonical Runtime Flow
1. Command/control ingress enters runtime.
2. Workspace and session context is resolved.
3. Governance and policy resolution is applied.
4. Enforcement executes and produces decision/evidence outputs.
5. Data and graph sinks persist accepted outcomes.

## Runtime Decomposition
- Architecture core: `architecture.md` (this document)
- Resolution semantics: `resolution.md`
- Enforcement semantics: `enforcement.md`
- Data sink integration: `data-sinks.md`

# Related Docs
- `docs/architecture/workspace/architecture.md`
- `docs/architecture/orchestration/architecture.md`
- `docs/reference/commands/surface.md`
