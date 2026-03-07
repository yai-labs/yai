# Repository Refoundation Closeout

## Official Repository Topology
Authoritative structure:
- `cmd/{yai,yai-core}`
- `include/yai/*`
- `lib/{core,exec,brain,protocol,platform,support}`
- `tests/{unit,integration,e2e,fixtures,shared}`
- `tools/*`
- `docs/*`

## Entry and Runtime Identity
Authoritative entrypoints:
- `cmd/yai/main.c`
- `cmd/yai-core/main.c`

Runtime composition baseline:
- `core` -> `exec` -> `brain` (teardown in reverse)

## Legacy Decommission Status
Legacy top-level domains are removed from authoritative topology:
- `boot/`, `root/`, `kernel/`, `engine/`, `mind/` removed.
- `runtime-protocol/` identity decommissioned and removed; protocol implementation is authoritative in `lib/protocol`.

## Build and Test Spine
Root Makefile is the authoritative build/test orchestrator.
Primary bins:
- `build/bin/yai`
- `build/bin/yai-core`

Primary tests:
- `make test-unit`
- `make test-integration`
- `make test-e2e`
- `make test`

## What Was Removed or Absorbed
- protocol runtime implementation absorbed into `lib/protocol`
- support primitives absorbed into `lib/support`
- core/exec/brain modules now built from `lib/*` authoritative paths
- brain tests moved out of `mind/tests_c` into `tests/*`

## Residual Legacy (Intentional)
Residual legacy remains only as compatibility leaves outside removed top-level domains:
- `cmd/legacy/yai-mind/main.c` compatibility daemon entry used by integration smoke
- deprecated `tests/domains/*` placeholders retained as migration markers

## Bridge Toward `yai-law` Refoundation
Packaging changed; semantics remain governed.
For `yai-law` follow-up:
- preserve authority/workspace/session invariants
- preserve separation of planes (`core`, `exec`, `brain`)
- reflect new package topology in contract pointers and trace docs
- remove references to legacy package identities as primary implementation paths
