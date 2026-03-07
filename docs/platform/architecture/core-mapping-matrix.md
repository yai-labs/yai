# Core Mapping Matrix

| Legacy Path | Target Path | Action | Semantic Responsibility | Dependency Impact | Follow-up |
|---|---|---|---|---|---|
| `boot/src/bootstrap.c` | `lib/core/lifecycle/bootstrap.c` | `move` | Startup lifecycle | `core` now owns plane spawn bootstrap | Entrypoint convergence in phase 8 |
| `boot/src/preboot.c` | `lib/core/lifecycle/preboot.c` | `move` | Runtime preflight/layout | `cmd/yai-core` can call lifecycle directly | Path authority cleanup wave |
| `root/src/control_transport.c` | `lib/core/dispatch/control_transport.c` | `move` | Control transport dispatch IO | Shared with `root` + `kernel` via core | Split protocol/platform details later |
| `root/src/core/commands/root_command_dispatch.c` | `lib/core/dispatch/command_dispatch.c` | `move+rename` | Root command dispatch | Dispatch surface consolidated in core | Extract kernel relay details later |
| `kernel/src/core/yai_session.c` | `lib/core/session/session.c` | `move+rename` | Session lifecycle | Session logic removed from kernel packaging | Internal header cleanup |
| `kernel/src/core/commands/yai_session_reply.c` | `lib/core/session/session_reply.c` | `move+rename` | Session reply shaping | Core session reply module | Reply model cleanup later |
| `kernel/src/core/commands/yai_session_utils.c` | `lib/core/session/session_utils.c` | `move+rename` | Session/workspace helpers | Core session utils now reusable | Split workspace helpers to `workspace/` |
| `kernel/src/core/project_tree.c` | `lib/core/workspace/project_tree.c` | `move+rename` | Workspace tree introspection | Workspace module under core | Replace stdout scan with structured output |
| `kernel/src/enforcement/enforcement.c` | `lib/core/enforcement/enforcement.c` | `move+rename` | Envelope enforcement baseline | Enforcement pulled into core | Align error constants with support/errors |
| `boot/include/bootstrap.h` | `include/yai/core/lifecycle.h` | `merge` | Public lifecycle boundary | Legacy include now wrapper | Remove wrapper in legacy removal wave |
| `boot/include/preboot.h` | `include/yai/core/lifecycle.h` | `merge` | Public lifecycle boundary | Legacy include now wrapper | Remove wrapper in legacy removal wave |
| `root/include/control_transport.h` | `include/yai/core/dispatch.h` | `merge` | Public dispatch boundary | Legacy include now wrapper | Remove wrapper in legacy removal wave |
| `root/include/root_command_dispatch.h` | `include/yai/core/dispatch.h` | `merge` | Public dispatch boundary | Legacy include now wrapper | Remove wrapper in legacy removal wave |
| `root/include/ws_id.h` | `include/yai/core/workspace.h` | `merge` | Workspace id validation | Legacy include now wrapper | Optionally move ID grammar to support layer |
| `kernel/include/yai_session.h` | `include/yai/core/session.h` | `keep-temporary` | Session public model | Transitional wrapper still points to kernel header | Replace with native core session header |
| `kernel/include/yai_session_internal.h` | `lib/core/session/internal.h` | `keep-internal` | Session internal helpers | Remains internal-only | Deduplicate with legacy header and move |
| `kernel/include/yai_vault.h` | `include/yai/core/vault.h` | `rename-wrapper` | Vault ABI/public authority view | Public include normalized | Full ownership move in later wave |
| `kernel/include/yai_events.h` | `include/yai/core/events.h` | `rename` | Event taxonomy | Event symbols now under core headers | Remove duplicate legacy declaration |
| `kernel/src/bin/workspace_kernel_main.c` | `cmd/yai-core/main.c` | `keep-temporary` | Runtime daemon entry | Legacy main still active in compatibility path | Merge into unified runtime entry |
| `root/src/yai_root_server.c` | `cmd/yai-core/main.c` | `keep-temporary` | Root daemon loop | Still built for legacy compatibility | Merge into unified runtime entry |
| `kernel/src/core/commands/yai_control_call.c` | `lib/core/dispatch/*` | `keep-temporary` | Control-call grammar/handlers | Still compiled by kernel legacy path | Split command grammar and move |
| `kernel/src/core/fsm.c` | `lib/core/enforcement/*` | `keep-temporary` | Sovereign state transition policy | Still kernel-owned today | Rename and merge into core enforcement |
