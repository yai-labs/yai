# Architecture Docs

Canonical architecture documentation for the unified platform.

## Canonical Sources (one primary source per domain)

- System overview: `overview/repository-scope.md`
- Runtime: `runtime/runtime-architecture.md`
- Workspace: `workspace/workspace-architecture.md`
- Governance: `governance/governance-architecture.md`
- Distributed runtime: `distributed-runtime/distributed-runtime-architecture.md`
- Protocol: `protocol/secure-overlay-transport-plane-architecture.md`
- Data runtime: `data-runtime/canonical-data-plane-architecture.md`
- Intelligence runtime: `intelligence-runtime/daemon-local-runtime-architecture.md`
- System theory boundary: `system-theory/README.md`

## Workspace 5-Spine Model

- `workspace/workspace-architecture.md`
- `workspace/workspace-boundaries-and-containment-architecture.md`
- `workspace/workspace-state-and-lifecycle-architecture.md`
- `workspace/workspace-security-and-scope-architecture.md`
- `workspace/workspace-peer-and-distribution-architecture.md`

## Rules

- New architecture docs must extend an existing canonical source before creating a new file.
- Satellite docs are allowed only when they do not duplicate canonical source-of-truth semantics.
- Migration or historical architecture materials belong in `docs/archive/`.
