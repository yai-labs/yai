# K-8 Kernel Lifecycle Core

## Canonical lifecycle states

- `booting`
- `initializing`
- `ready`
- `degraded`
- `recovery`
- `shutting_down`
- `halted`

## Readiness gates

- proc, sched, mm, fs, ipc, security
- container primitives
- audit
- registry
- session admission
- policy/grants

## Canonical transitions

- booting -> initializing
- initializing -> ready | degraded
- ready -> degraded | shutting_down
- degraded -> recovery | shutting_down
- recovery -> ready | shutting_down
- shutting_down -> halted

## Kernel-owned primitives

- `yai_kernel_boot_begin(...)`
- `yai_kernel_mark_subsystem_ready(...)`
- `yai_kernel_enter_ready(...)`
- `yai_kernel_enter_degraded(...)`
- `yai_kernel_enter_recovery(...)`
- `yai_kernel_begin_shutdown(...)`
- `yai_kernel_halt(...)`

## Lifecycle-based gating helpers

- `yai_kernel_can_admit_sessions(...)`
- `yai_kernel_can_create_container(...)`
- `yai_kernel_can_bind_container(...)`
- `yai_kernel_can_issue_grants(...)`

## Ownership lock

Only kernel lifecycle APIs can mutate global lifecycle state.
Higher layers can request/report/observe, but cannot declare global readiness,
skip degraded/recovery constraints, or bypass lifecycle gates.
