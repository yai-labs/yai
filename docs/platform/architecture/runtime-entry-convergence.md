# Runtime Entry Convergence

## Authoritative Entrypoints
The unified runtime now converges on:
- `cmd/yai/main.c` as operator CLI and multiplexer
- `cmd/yai-core/main.c` as unified runtime entrypoint

## Binary Topology
Primary operational binaries:
- `build/bin/yai`
- `build/bin/yai-core`

Legacy binaries (`yai-boot`, `yai-root-server`, `yai-kernel`, `yai-engine`, `yai-mind`) remain compatibility-only and explicitly deprecated.

## `yai` Role
`yai` is the user/operator command surface.
Current behavior:
- `yai up` -> delegates to `yai-core --run`
- `yai status` -> delegates to `yai-core --status`
- `yai brain-check` -> delegates to `yai-core --brain-check`
- `yai core ...` -> pass-through to `yai-core`
- unknown subcommands -> pass-through to `yai-core`

## `yai-core` Role
`yai-core` is the runtime composition entrypoint.

Supported modes:
- `--run` (default when no args): compose runtime baseline
- `--status`: probe readiness/status
- `--preflight`: core preboot/layout checks
- `--exec-probe`: execution plane probe
- `--brain-check`: cognition/brain smoke
- `--legacy-*`: deprecated compatibility launchers

## Runtime Composition Order
Current baseline order in `yai-core --run`:
1. `core` preflight (`yai_run_preboot_checks`)
2. `core` runtime layout (`yai_ensure_runtime_layout`)
3. `exec` attachment probe (`yai_exec_runtime_probe`)
4. `brain` init (`yai_mind_init`)
5. ready state report
6. shutdown in reverse priority (`yai_mind_shutdown`)

## Legacy Main Classification
- `boot/src/yai_boot_main.c`: keep-temporary + deprecate, lifecycle logic already mirrored in core lifecycle APIs.
- `root/src/yai_root_server.c`: keep-temporary + deprecate, control-plane logic targeted to `lib/core/dispatch` composition.
- `kernel/src/bin/workspace_kernel_main.c`: keep-temporary + deprecate, session/control handling targeted to `lib/core` composition.
- `engine/src/main.c`: keep-temporary + deprecate, execution wiring moved into `lib/exec`.
- `mind/src/main.c`: keep-temporary + deprecate, brain lifecycle/wiring moved to `lib/brain` and composed by `yai-core`.

## Makefile Convergence
Root Makefile now treats:
- `build` as primary build (`yai`, `yai-core`)
- `legacy-build` as compatibility plane build

Legacy plane-specific targets remain available but marked deprecated in output.

## Temporary Compatibility Remaining
- `yai-core --legacy-root|--legacy-kernel|--legacy-engine|--legacy-mind`
- legacy per-plane binaries and targets for transition validation
- `mind/src/main.c` and per-domain legacy mains still present for compatibility

## Deferred Items
- remove legacy entrypoint executables from default dist bundle
- fully absorb remaining per-plane runtime loops into `lib/core`, `lib/exec`, `lib/brain`
- remove `--legacy-*` modes after parity gates are complete
