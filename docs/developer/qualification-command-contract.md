# Qualification Command Contract (QW-1)

## Purpose

Define one canonical command grammar for qualification across owner runtime, peer daemon, SDK/CLI surfaces, and integration scripts.

This document is the baseline contract for the QW waves:

1. `QW-1` LAN baseline
2. `QW-2` secure peering wave
3. `QW-3` scale wave
4. `QW-4` real-flow wave

## Contract Levels

- `stable`: command/shape already used in automated tests.
- `baseline`: v1 contract fixed for qualification; implementation may still be bridge-driven.
- `provisional`: accepted name/intent; full implementation in next wave.

## Owner-Side Contract

Canonical operator namespace (CLI-facing):

- `yai up` (`stable` in this repo)
- `yai down` (`stable` in this repo)
- `yai workspace create <ws>` (`baseline` via runtime control call)
- `yai workspace set <ws>` (`baseline` via runtime control call)
- `yai workspace domain-set --family ... --specialization ...` (`baseline`)
- `yai workspace governance-apply --overlays ...` (`baseline`)
- `yai source enroll ...` (`baseline` via runtime control call)
- `yai source attach ...` (`baseline` via runtime control call)
- `yai source list ...` (`baseline`, mapped to `yai.workspace.query source.peer`)
- `yai source status ...` (`baseline` via runtime control call)
- `yai source inspect ...` (`baseline`, mapped to source inspect/query surfaces)
- `yai workspace query ...` (`stable` in runtime control layer)
- `yai workspace graph summary` (`baseline`, mapped to graph read surfaces)

Current implementation note for this repo (`yai`):

- fallback binary `build/bin/yai` exposes direct lifecycle CLI (`up/down`) and runtime ingress;
- qualification scripts call owner operations through `yai.control.call.v1` over runtime socket.

## Peer-Side Contract

Canonical operator namespace (CLI-facing):

- `yai-daemon init ...` (`provisional`)
- `yai-daemon start` (`provisional`)
- `yai-daemon stop` (`provisional`)
- `yai-daemon status` (`baseline`, via health state + runtime status surfaces)
- `yai-daemon enroll ...` (`baseline`, runtime-mediated)
- `yai-daemon attach ...` (`baseline`, runtime-mediated)
- `yai-daemon binding add ...` (`baseline`, manifest-driven in v1)
- `yai-daemon binding list` (`baseline`, bindings state file)
- `yai-daemon scan` (`baseline`, scan loop)
- `yai-daemon emit` (`baseline`, delivery loop)
- `yai-daemon spool list` (`baseline`, spool dir state)
- `yai-daemon retry-drain` (`baseline`, delivery retry pass)

Current implementation note for this repo (`yai`):

- daemon runs foreground runtime; enroll/attach/emit/status happen through owner control calls;
- qualification scripts validate state through `state/health.v1.json`, `state/bindings.v1.json`, and spool folders.

## Qualification Mapping Rules

- Do not introduce ad-hoc command names per script.
- Reuse runtime command IDs and SDK/CLI naming above.
- For LAN wave in `yai` repo, use runtime control call bridge as canonical execution backend.
- Keep owner/peer boundaries explicit:
  - owner truth and orchestration on `yai`
  - peer acquisition/spool/retry on `yai-daemon`

## QW-1 Scope Boundary

QW-1 qualifies LAN baseline only:

- owner + peer connectivity in local/trusted network
- source enroll/attach/emit/status path
- workspace multi-peer baseline (3 peers)
- replay/backlog/retry and overlap visibility baseline

Not in QW-1:

- secure overlay peering qualification
- scale/fleet simulation qualification
- real-flow qualification

These are explicit next waves and must reuse this contract without renaming the grammar.
