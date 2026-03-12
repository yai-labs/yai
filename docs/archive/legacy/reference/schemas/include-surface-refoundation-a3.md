# Include Surface Refoundation (A3)

This document defines the canonical public header topology for `include/yai/`.

## Canonical Public Namespaces

- `api/`
- `runtime/`
- `edge/`
- `mesh/`
- `orchestration/`
- `agents/`
- `providers/`
- `knowledge/`
- `graph/`
- `governance/`
- `data/`
- `protocol/`
- `platform/`
- `support/`

## Migration Mapping (Legacy -> Canonical)

- `core/` -> `runtime/`
- `daemon/` -> `edge/`
- `exec/` -> `orchestration/` + `agents/` + mesh/runtime surfaces
- `governance/` -> `governance/`

## A3 Cutover Policy

- new public include usage must prefer canonical namespaces
- `include/yai/governance/` remains compatibility-only during migration
- governance path-level cutover is active (`<yai/governance/...>`)
- type-level rename (`yai_law_*` -> `yai_governance_*`) is deferred to a
  dedicated follow-up migration slice
