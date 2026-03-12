# Container Public Surface (Canonical)

Container is the canonical contained operational domain.
Workspace is legacy migration source only.

Core C-3 surface:
- `root.h`: projected-root and backing-store model.
- `paths.h`: container-relative path context/resolution and traversal gate.
- `mounts.h`: governed mount objects and mount-set visibility model.

Core C-4 surface:
- `session.h`: container-bound interactive session primitives (`bind/unbind/rebind/enter/leave`).
- Bound session views are container-scoped for root/path/runtime access.
