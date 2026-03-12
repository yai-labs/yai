# Container Domain (Canonical)

`container/` is the canonical contained operational domain in YAI.

## Architectural role

- Container owns operational domain identity, lifecycle, root, state, services, and scoped views.
- Kernel owns privileged roots: containment primitives, session admission root, grants validity root, system lifecycle root.
- Orchestration coordinates work on/among containers but does not own container truth.
- Daemons attach/bind to containers; they do not replace container domain ownership.

## Filesystem model (C-3)

- Container runs on a projected root, not directly on host filesystem.
- The model is explicit:
  - host filesystem
  - container backing store
  - projected operational root
  - authorized external mounts
- Path resolution is container-relative by default and rejects traversal escape outside projected root.
- Mounts are governed objects with policy, visibility, attachability and class.

## Interactive domain model (C-4)

- A bound session becomes container-scoped operational context, not host-scoped context.
- Session enter/leave/rebind primitives expose container root/path/runtime views by default.
- Escape from the container domain is explicit and policy-governed.

## Legacy status

`workspace` is legacy migration vocabulary only.
Operational work, scoped session binding, and runtime views are container-centric.
