# Workspace Containment Levels

## Levels
- `logical`: context and routing level containment only.
- `scoped`: structured workspace-owned state/path/trace/artifact containment.
- `isolated`: stronger execution channel/process/socket/resource controls (partially future).
- `sandboxed`: backend-enforced hard isolation target (future backend).

## Current baseline
Current runtime can reliably express `logical` and `scoped`; `isolated/sandboxed` are modeled and hook-ready.
