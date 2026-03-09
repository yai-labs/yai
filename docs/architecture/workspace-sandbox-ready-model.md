# Workspace Sandbox-Ready Model

## Backend modes
- `none` (current default)
- `structured_local` (current structure-first mode)
- `process_isolated` (future)
- `container_backed` (future)
- `sandbox_backed` (future)

The runtime exposes backend mode in envelope metadata so future containment backends can be integrated without changing workspace contracts.
