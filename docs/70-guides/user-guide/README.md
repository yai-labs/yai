# User Guide Pointers

This folder contains editorial pointers for navigation.
Normative law/specs remain in `deps/yai-specs/**`.

## Canonical spec pointers

- Engine cortex:
  - `deps/yai-specs/specs/engine/engine_cortex.v1.json`
- Control/events:
  - `deps/yai-specs/specs/control/control_plane.v1.json`
- Graph:
  - `deps/yai-specs/specs/graph/GRAPH_V1.md`
  - `deps/yai-specs/specs/graph/graph.v1.json`
- Providers trust:
  - `deps/yai-specs/specs/providers/PROVIDERS_TRUST.md`
  - `deps/yai-specs/specs/providers/providers.v1.json`
- RPC/CLI surface:
  - `deps/yai-specs/specs/cli/CLI_PUBLIC_INTERFACE.md`
  - `deps/yai-specs/specs/cli/commands.v1.json`

## Boundary rule

If a change modifies contract/spec behavior, it belongs upstream in `yai-specs`.
This folder should remain explanatory/navigation-only.
