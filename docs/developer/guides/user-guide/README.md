# User Guide Pointers

This folder contains editorial pointers for navigation.
Normative law/specs remain in `deps/law/**`.

## Canonical spec pointers

- Engine cortex:
  - `deps/law/runtime/engine/schema/engine_cortex.v1.json`
- Control/events:
  - `deps/law/contracts/control/schema/control_plane.v1.json`
- Graph:
  - `deps/law/runtime/mind/graph/notes/GRAPH_V1.md`
  - `deps/law/runtime/mind/graph/schema/graph.v1.json`
- Providers trust:
  - `deps/law/contracts/providers/notes/PROVIDERS_TRUST.md`
  - `deps/law/contracts/providers/schema/providers.v1.json`
- RPC/CLI surface:
  - `deps/law/contracts/cli/notes/CLI_PUBLIC_INTERFACE.md`
  - `deps/law/registry/commands.v1.json`

## Boundary rule

If a change modifies contract/spec behavior, it belongs upstream in `law`.
This folder should remain explanatory/navigation-only.
