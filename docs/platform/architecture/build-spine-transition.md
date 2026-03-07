# Build Spine Transition

## Purpose
This document defines the transition build spine introduced after the repository refoundation blueprint.

## Minimal binary topology (active)
The active minimal binary topology is:
- `build/bin/yai`
- `build/bin/yai-core`

No additional binaries are introduced by default for `exec` or `brain`.

## Entrypoints
- `cmd/yai/main.c`
  - operator/CLI entrypoint,
  - delegates to `yai-core` for runtime-oriented execution.
- `cmd/yai-core/main.c`
  - unified runtime entrypoint in transition mode,
  - provides controlled compatibility flags for legacy runtime binaries.

## Root Makefile role
Root `Makefile` is now the primary build orchestrator for the new spine:
- `all` -> builds `yai` and `yai-core`
- `yai` -> builds `build/bin/yai`
- `yai-core` -> builds `build/bin/yai-core`
- `test` -> transition baseline test target
- `clean` -> removes build artifacts

Legacy targets (`build`, `boot`, `root`, `kernel`, `engine`, `mind`) remain available as controlled compatibility paths during migration waves.

## Build artifact convention
The transition Makefile standardizes artifact semantics:
- `build/bin/` -> executables
- `build/obj/` -> object files
- `build/lib/` -> static/shared archive staging
- `build/test/` -> test binaries/output staging

`build/` is artifacts only; it does not encode source topology semantics.

## What is still legacy
Still legacy in this phase:
- top-level source domains (`boot/`, `root/`, `kernel/`, `engine/`, `mind/`, `runtime-protocol/`),
- their internal build files and mains,
- domain-level source layout.

This phase does not perform mass movement of those modules.

## What follows next
Subsequent waves migrate module ownership gradually into:
- `lib/core`
- `lib/exec`
- `lib/brain`
- `lib/protocol`
- `lib/platform`
- `lib/support`

The current build spine exists to make that migration incremental and testable.
