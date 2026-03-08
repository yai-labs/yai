# Repository Refoundation Blueprint

## Executive summary
This blueprint defines the target repository grammar for `yai/` before any large code movement.
The target state is a runtime-first C repository with:
- one unified build spine,
- two binaries (`yai`, `yai-core`),
- modular semantic domains (`core`, `exec`, `brain`),
- explicit cross-cutting foundations (`protocol`, `platform`, `support`).

This document is intentionally prescriptive for packaging and dependency direction, while preserving normative invariants owned by `law`.

## Why the old topology is no longer the right packaging model
The legacy top-level domains (`boot/`, `root/`, `kernel/`, `engine/`, `mind/`, `runtime-protocol/`) reflect historical implementation phases, not the runtime model we now operate.

Problems in the old packaging model:
- package names encode history instead of responsibility,
- cross-cutting boundaries are hard to enforce,
- duplicated build and include semantics emerge naturally,
- binary topology is implied by folders rather than declared by runtime design.

The new packaging model encodes stable runtime responsibilities and enforces dependency direction.

## Target repository topology
Baseline target topology:

- `cmd/`
  - `yai/`
  - `yai-core/`
- `include/yai/`
  - `api/`
  - `core/`
  - `exec/`
  - `brain/`
  - `protocol/`
  - `platform/`
  - `support/`
- `lib/`
  - `core/`
  - `exec/`
  - `brain/`
  - `protocol/`
  - `platform/`
  - `support/`
  - `third_party/`
- `tests/`
  - `unit/`
  - `integration/`
  - `e2e/`
  - `fixtures/`
  - `shared/`
- `docs/`
- `data/`
- `tools/`
- `build/`
- `.github/`

Refinements are allowed in later waves only if they preserve this grammar.

## Binary topology (minimal model)
Canonical binary topology for this refoundation:
- `cmd/yai`: CLI and operator entrypoint.
- `cmd/yai-core`: unified runtime entrypoint.

Explicit constraints:
- `exec` and `brain` are internal runtime modules by default.
- no automatic additional binaries for `exec` or `brain`.
- a folder under `lib/` does not imply an executable under `cmd/`.

## Domain model: core, exec, brain
### core
`core` is the sovereign runtime plane. It owns:
- authority model enforcement,
- workspace/runtime sovereignty,
- session lifecycle,
- dispatch and control routing,
- baseline enforcement and lifecycle bootstrap.

### exec
`exec` is the execution plane. It owns:
- runtime execution paths,
- gates and execution controls,
- bridge concerns toward resources and execution environments,
- controlled interaction with external systems.

`exec` does not own authority semantics.

### brain
`brain` is the cognitive plane. It owns:
- cognition orchestration,
- memory model and retrieval behavior,
- cognitive provider bridges,
- brain transport concerns,
- cognitive lifecycle concerns.

`brain` is a module of unified runtime by default, not a separate process model by default.

## Foundation layers: protocol, platform, support
### protocol
Cross-cutting runtime protocols and codecs used by core/exec/brain.

### platform
OS/platform wrappers and environmental adapters (FS, UDS, clock, process).

### support
Reusable utilities and low-level foundations (ids, logging, strings, alloc helpers, shared error helpers).

These layers are foundations, not residual buckets.

## Mapping from historical domains to new structure
Historical roles are preserved semantically but no longer as top-level packaging.

- `boot` responsibilities converge into `lib/core/lifecycle`.
- `root` and `kernel` responsibilities converge into `lib/core/*` with preserved authority and dispatch semantics.
- `engine` converges into `lib/exec/*`.
- `mind` converges into `lib/brain/*` with runtime-integrated cognition model.
- `runtime-protocol` converges into `lib/protocol/*`.

## Dependency direction rules
Top-level dependency direction:
- `cmd/*` -> `lib/{core,exec,brain,protocol,platform,support}`
- `lib/core` -> `lib/{protocol,platform,support}`
- `lib/exec` -> `lib/{protocol,platform,support}` and `lib/core` APIs where explicitly approved
- `lib/brain` -> `lib/{protocol,platform,support}` and `lib/core` APIs where explicitly approved
- `lib/protocol` -> `lib/support`
- `lib/platform` -> `lib/support`
- `lib/support` -> no internal domain dependency

Forbidden defaults:
- `support` depending on `core/exec/brain`
- `platform` depending on `core/exec/brain`
- circular dependencies between `core`, `exec`, `brain`

## Header/public API rules
Public tree is rooted at `include/yai/*`.

Rules:
- headers in `include/yai/...` are consumable public/semipublic boundaries.
- private implementation headers must remain local (`internal.h` pattern under `lib/...`).
- `internal.h` files must not be re-exported from public include tree.
- each public header namespace must map to one domain owner (`core`, `exec`, `brain`, `protocol`, `platform`, `support`, `api`).

## File placement rules
Placement is determined by semantic ownership, not by historical path.

- lifecycle/bootstrap/authority/session/dispatch/enforcement -> `lib/core/*`
- runtime execution, gating, external execution bridges -> `lib/exec/*`
- cognition/memory/provider-cognitive paths -> `lib/brain/*`
- protocol codecs/rpc envelopes/binary contracts -> `lib/protocol/*`
- OS wrappers/clock/fs/uds/process adaptation -> `lib/platform/*`
- ids/logger/strings/arena/helpers -> `lib/support/*`

If ownership is mixed, split by responsibility.

## Legacy decommissioning policy
Legacy top-level domains are decommissioned only after mapped migration is complete and validated.

Allowed migration actions (mandatory classification):
- `keep`
- `move`
- `rename`
- `merge`
- `split`
- `remove`
- `move-to-tests`

No mass deletion without mapped replacement and validation path.

## Open questions / deferred items
Deferred to next waves:
- exact file-level split between `core` and `exec` for ambiguous runtime bridge files,
- public API stabilization granularity under `include/yai/api/`,
- final granularity of integration vs e2e test boundaries after module migration,
- tooling path migration specifics under `tools/` once `cmd/` and `lib/` paths are materialized.

## Sequencing notes for next refactor waves
- Wave 2: build spine + `cmd/` minimal binary entrypoints.
- Wave 3: public include tree convergence.
- Waves 4-7: foundations -> core -> exec -> brain migration.
- Wave 8: runtime entry convergence.
- Wave 9: tests/tooling realignment.
- Wave 10: legacy top-level removal and hardening.

This blueprint is the decision baseline for all subsequent file movements.
