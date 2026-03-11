# Control Plane (Core Sovereign Model)

This document is normative.
Control-plane semantics are modeled as `core` authority semantics.

## Ontology

- `core`: sovereign authority, lifecycle, dispatch, enforcement baseline
- `exec`: execution and external-effect plane
- `brain`: cognitive plane attached under governance
- `protocol/platform/support`: cross-cutting layers, not authority domains

## Compatibility aliases

Historical labels may appear in command names and legacy payloads:
- `ingress` → core-dispatch historical alias
- `root`/`kernel` → core-sovereignty historical aliases
- `engine` → exec historical alias
- `mind` → brain historical alias

Aliases are compatibility surfaces only.

## Canonical rules

1. Clients do not bypass control contracts.
2. RPC/control envelopes remain the authoritative command channel.
3. External effects require explicit governance and compliance context.
4. Workspace-scoped authority and traceability are mandatory.
5. Replies use `yai.exec.reply.v1` regardless of legacy naming.
