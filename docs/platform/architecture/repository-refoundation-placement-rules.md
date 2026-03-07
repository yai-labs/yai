# Repository Refoundation Placement Rules

## Purpose
These rules determine where files belong in the refounded repository and which migration action is valid.

## 1. Domain ownership rules
### `core`
Place in `core` only if the file owns sovereign runtime behavior:
- authority,
- workspace/runtime sovereignty,
- session lifecycle,
- dispatch,
- enforcement,
- runtime lifecycle/bootstrap.

### `exec`
Place in `exec` only if the file owns execution-plane behavior:
- runtime execution paths,
- gate controls,
- execution bridge toward external resources/environments.

`exec` must not own authority model decisions.

### `brain`
Place in `brain` only if the file owns cognitive behavior:
- cognition,
- memory,
- cognitive provider bridge,
- brain-specific transport/lifecycle.

`brain` remains runtime-internal by default.

### `protocol`
Place protocol contracts, codecs, rpc envelopes, binary protocol assets.

### `platform`
Place OS/environment adapters and wrappers.

### `support`
Place generic reusable primitives with no dependency on domain semantics.

## 2. Public header vs internal header rules
Use `include/yai/...` only for stable or intentionally consumable boundaries.

Use `internal.h` when the header:
- exposes implementation details,
- leaks module-private structs,
- is only needed by files in the same module,
- would create forbidden upward dependencies if made public.

Never re-export `internal.h` through public include trees.

## 3. Rename vs move vs split vs merge
### `move`
Use when file responsibility remains identical and only packaging changes.

### `rename`
Use when name encodes legacy semantics no longer valid in target topology.

### `split`
Use when one file mixes multiple semantic owners (e.g., core + protocol logic).

### `merge`
Use when several legacy files represent one cohesive target boundary.

### `remove`
Use only when file is obsolete and replaced or intentionally retired.

### `move-to-tests`
Use for mocks, fixtures, test harness helpers that must not stay in runtime modules.

## 4. File placement constraints
- No top-level legacy domain name can be used as new canonical packaging target.
- No file under `lib/` implies a corresponding binary.
- `cmd/` remains strictly minimal (`yai`, `yai-core`) unless future decision explicitly extends topology.
- `build/` contains artifacts only; no source semantics are allowed under `build/`.

## 5. Dependency constraints tied to placement
- `support` cannot depend on domain modules.
- `platform` cannot depend on `core/exec/brain` semantics.
- `protocol` cannot import domain behavior.
- `core`, `exec`, `brain` may depend on foundations, but cross-domain dependencies require explicit boundary APIs.
- cyclic dependencies are forbidden.

## 6. Migration action declaration requirement
For each touched file in refactor waves 2-10, one explicit action must be declared:
- `keep`, `move`, `rename`, `merge`, `split`, `remove`, `move-to-tests`.

Missing action classification is considered migration debt and blocks completion.

## 7. Deferred precision policy
If file-level ownership is ambiguous during planning:
- do not invent false precision,
- mark follow-up required,
- keep migration intent explicit at domain level,
- resolve ambiguity in the next wave before movement.
