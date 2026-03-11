---
id: ARCH-COMP-ROOT
status: historical
effective_date: 2026-02-19
revision: 1
owner: runtime
law_refs:
  - ../law/foundation/axioms/A-002-authority.md
  - ../law/foundation/invariants/I-003-governance.md
---

# Root Component

> Historical archive: this component page describes pre-cutover topology.
> Current runtime model is single-binary `yai` ingress; root/kernel/engine are internal semantics, not standalone public components.

## Role

Machine ingress plane handling envelope-level guardrails and request routing into governed runtime planes.

## Current Implementation Status

implemented

## Interfaces and Entry Points

- `root/src/yai_root_server.c`
- `root/src/control_transport.c`
- Socket path in current code: `~/.yai/run/root/root.sock`

## Authority and Boundary Rules

- Must require handshake before effectful commands.
- Must not mutate workspace authority state.
- Must preserve trace/workspace identifiers in forwarded responses.

## Traceability

- ADR refs: `docs/program/adr/ADR-001-single-runtime.md`, `docs/program/adr/ADR-002-root-entrypoint.md`
- Runbook refs: `docs/program/milestone-packs/runtime-baselines/root-hardening.md`
- MP refs: `docs/program/milestone-packs/root-hardening/MP-ROOT-HARDENING-0.1.0.md`, `docs/program/milestone-packs/root-hardening/MP-ROOT-HARDENING-0.1.1.md`
- L0 anchors: `../law/contracts/protocol/include/transport.h`, `../law/contracts/protocol/include/auth.h`

## Known Drift / Gaps

- Path naming drift vs some docs/tooling (`root.sock` vs `control.sock`).

## Next Alignment Steps

- Normalize public socket path naming across implementation and docs.
