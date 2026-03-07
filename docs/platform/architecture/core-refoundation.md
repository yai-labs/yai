# Core Refoundation

## Scope
This wave absorbs sovereign runtime responsibilities from legacy `boot/`, `root/`, and `kernel/` into:

- `lib/core/`
- `include/yai/core/`

The goal is packaging refoundation, not semantic flattening.

## Core Domain Definition
`core` is restricted to:

- authority
- workspace sovereignty
- session lifecycle
- control dispatch
- enforcement baseline
- startup lifecycle

`core` explicitly excludes:

- execution gates/runtime externals (`exec`)
- cognition/memory/provider concerns (`brain`)
- rpc codec/wire framing (`protocol`)
- os/fs/uds/clock adapters (`platform`)
- neutral primitives (`support`)

## Implemented Structure

- `lib/core/authority/`
- `lib/core/workspace/`
- `lib/core/session/`
- `lib/core/dispatch/`
- `lib/core/enforcement/`
- `lib/core/lifecycle/`
- `lib/core/internal.h`

## Effective Module Migration

- `boot/src/bootstrap.c` -> `lib/core/lifecycle/bootstrap.c`
- `boot/src/preboot.c` -> `lib/core/lifecycle/preboot.c`
- `root/src/control_transport.c` -> `lib/core/dispatch/control_transport.c`
- `root/src/core/commands/root_command_dispatch.c` -> `lib/core/dispatch/command_dispatch.c`
- `kernel/src/core/yai_session.c` -> `lib/core/session/session.c`
- `kernel/src/core/commands/yai_session_reply.c` -> `lib/core/session/session_reply.c`
- `kernel/src/core/commands/yai_session_utils.c` -> `lib/core/session/session_utils.c`
- `kernel/src/core/project_tree.c` -> `lib/core/workspace/project_tree.c`
- `kernel/src/enforcement/enforcement.c` -> `lib/core/enforcement/enforcement.c`

## Public Core Surface

- `include/yai/core/authority.h`
- `include/yai/core/workspace.h`
- `include/yai/core/session.h`
- `include/yai/core/dispatch.h`
- `include/yai/core/enforcement.h`
- `include/yai/core/lifecycle.h`
- `include/yai/core/events.h`
- `include/yai/core/vault.h`

Legacy headers in `boot/include` and `root/include` are now compatibility wrappers pointing to `include/yai/core/*`.

## Ambiguous Legacy Cases (classification)

- `kernel/src/bin/workspace_kernel_main.c`: `keep-temporary` as legacy runtime entrypoint; target absorption into `cmd/yai-core/main.c`.
- `root/src/yai_root_server.c`: `keep-temporary` as legacy control-plane daemon; target absorption into `cmd/yai-core/main.c` dispatch runtime path.
- `kernel/src/core/commands/yai_control_call.c`: `keep-temporary` in kernel command handlers; candidate target `lib/core/dispatch` after control-call grammar extraction.
- `kernel/src/core/fsm.c`: `keep-temporary` in kernel; semantic target `lib/core/enforcement` (state transition policy) after naming cleanup.

## Build Impact

- Root `Makefile` now builds `build/lib/libyai_core.a` from `lib/core/*`.
- `cmd/yai-core/main.c` now imports `yai/core/lifecycle.h` and runs `--preflight` checks through core lifecycle.
- `boot`, `root`, and `kernel` Makefiles now compile migrated sources from `lib/core/*`.

## Intentional Limits

- No full entrypoint convergence in this wave.
- No `exec` or `brain` refoundation in this wave.
- No destructive removal of legacy top-level directories in this wave.
