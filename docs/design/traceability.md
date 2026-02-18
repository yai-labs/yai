# Traceability Map

This is the navigable index that prevents “docs drift”.
It maps frozen decisions and delivery artifacts to their proof.

## How to use this map
- Add a row when a new ADR or Runbook is introduced.
- Add MP rows as you ship phases.
- Keep links as **repo-relative paths** (no external URLs required).

## Map (fill over time)

| Capability / Track | Spec anchors (L0) | ADR (L3) | Runbook (L4) | MP (L5) | Tests / Evidence (L6) |
|---|---|---|---|---|---|
| Root hardening | `deps/yai-specs/specs/protocol/include/transport.h`<br/>`deps/yai-specs/specs/protocol/include/errors.h` | `docs/design/adr/ADR-006-unified-rpc.md` *(example)* | `docs/runbooks/root-hardening.md` | `docs/milestone-packs/root-hardening/MP-ROOT-HARDENING-0.1.0.md` | `docs/test-plans/*` + CI logs + runbook commands |
| Workspaces lifecycle | `deps/yai-specs/contracts/boundaries/L1-kernel.md` | `docs/design/adr/ADR-007-workspace-isolation.md` *(example)* | `docs/runbooks/workspaces-lifecycle.md` | *(TBD)* | *(TBD)* |

Notes:
- Examples above are placeholders. Update paths if they differ in your repo.
- Do not invent new anchors: always anchor to `deps/yai-specs` paths.
