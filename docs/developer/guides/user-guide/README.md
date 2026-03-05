# User Guide Pointers

This folder contains editorial pointers for navigation.
Normative law/specs remain in `deps/yai-law/**`.

## Canonical spec pointers

- Engine cortex:
  - `deps/yai-law/runtime/engine/schema/engine_cortex.v1.json`
- Control/events:
  - `deps/yai-law/contracts/control/schema/control_plane.v1.json`
- Graph:
  - `deps/yai-law/runtime/mind/graph/notes/GRAPH_V1.md`
  - `deps/yai-law/runtime/mind/graph/schema/graph.v1.json`
- Providers trust:
  - `deps/yai-law/contracts/providers/notes/PROVIDERS_TRUST.md`
  - `deps/yai-law/contracts/providers/schema/providers.v1.json`
- RPC/CLI surface:
  - `deps/yai-law/contracts/cli/notes/CLI_PUBLIC_INTERFACE.md`
  - `deps/yai-law/registry/commands.v1.json`

## Boundary rule

If a change modifies contract/spec behavior, it belongs upstream in `yai-law`.
This folder should remain explanatory/navigation-only.
