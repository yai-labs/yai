# User Guide Pointers

This folder contains editorial pointers for navigation.
Normative law/specs remain in `deps/yai-law/**`.

## Canonical spec pointers

- Engine cortex:
  - `deps/yai-law/specs/engine/engine_cortex.v1.json`
- Control/events:
  - `deps/yai-law/specs/control/control_plane.v1.json`
- Graph:
  - `deps/yai-law/specs/graph/GRAPH_V1.md`
  - `deps/yai-law/specs/graph/graph.v1.json`
- Providers trust:
  - `deps/yai-law/specs/providers/PROVIDERS_TRUST.md`
  - `deps/yai-law/specs/providers/providers.v1.json`
- RPC/CLI surface:
  - `deps/yai-law/specs/cli/CLI_PUBLIC_INTERFACE.md`
  - `deps/yai-law/specs/cli/commands.v1.json`

## Boundary rule

If a change modifies contract/spec behavior, it belongs upstream in `yai-law`.
This folder should remain explanatory/navigation-only.
