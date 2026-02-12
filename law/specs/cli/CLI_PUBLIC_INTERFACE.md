# YAI CLI Public Interface (v1)

This document defines the canonical public interface for the `yai` control suite.
It is normative. Implementations must conform.

## Scope

- Applies to the `yai` binary (Rust) and machine clients invoking the same command semantics.
- GUI front-ends (YX) must call the same control-plane contracts via UDS; they do not redefine command meaning.

## Principles

1. Single source of truth: command semantics are defined here and in `commands.v1.json`.
2. Deterministic execution: commands must be reproducible and observable.
3. Layer boundaries remain (L1 kernel authority, L2 engine deterministic execution, L3 mind orchestration).
4. Control plane is authoritative: CLI is a client and must not bypass RPC.

## Global Conventions

### Naming
- Binary: `yai`
- Subcommands: `yai <group> <action>` or `yai <action>` for top-level lifecycle.
- Flags: `--kebab-case`

### Output modes
- Default: human-readable text
- Optional: `--json` where applicable

### Exit codes
- `0` success
- `1` generic failure
- `2` invalid arguments / contract violation
- `3` dependency missing
- `4` runtime not ready

### Default Workspace
- A workspace id (`--ws`) identifies a runtime instance.
- If omitted, `ws_default` is loaded from config.

### Control Plane Paths (Canonical)

- `~/.yai/run/<ws>/control.sock`
- `~/.yai/run/<ws>/lock`
- `~/.yai/run/<ws>/daemon.pid`
- `~/.yai/run/<ws>/session.json`
- `/tmp/yai_runtime_<ws>.sock`

## Command Groups

### Lifecycle
- `yai up`
- `yai down`
- `yai restart`

### Runtime Inspection
- `yai status`
- `yai logs`
- `yai monitor`
- `yai events`

### Control
- `yai providers`
- `yai sessions`
- `yai dsar`
- `yai chat`
- `yai shell`

### Memory / Graph
- `yai graph`
- `yai embed`

### Verification
- `yai verify core|full`
- `yai test smoke`

## Command Contracts (summary)

### `yai up`
Purpose: start stack (boot + engine + mind) for a workspace.

Usage:
- `yai up --ws <id> [--build] [--ai] [--no-engine] [--no-mind] [--detach] [--monitor] [--timeout-ms <n>]`

Notes:
- `--monitor` opens monitor flow after successful start.

### `yai monitor`
Purpose: live operator monitor stream for a workspace.

Usage:
- `yai monitor --ws <id>`

Notes:
- If daemon is available, monitor tails the event stream.
- In VSCode terminal environments, implementation may spawn an external terminal.

### `yai events`
Purpose: stream daemon events (NDJSON-derived event messages).

Usage:
- `yai events --ws <id>`

### `yai providers`
Purpose: provider discovery, trust, and attach lifecycle.

Usage:
- `yai providers --ws <id> discover`
- `yai providers --ws <id> list`
- `yai providers trust --id <id>|--endpoint <ep> --state <discovered|trusted|revoked>`
- `yai providers --ws <id> pair <id> <endpoint> <model>`
- `yai providers --ws <id> attach <id> [--model <m>]`
- `yai providers --ws <id> detach`
- `yai providers --ws <id> revoke <id>`
- `yai providers --ws <id> status`

### `yai dsar`
Purpose: data subject access request lifecycle.

Usage:
- `yai dsar --ws <id> request <export|erase> --subject <subject_ref>`
- `yai dsar --ws <id> status <request_id>`
- `yai dsar --ws <id> execute <request_id>`

### `yai chat`
Purpose: interact with chat service exposed by control plane.

Usage:
- `yai chat --ws <id> list`
- `yai chat --ws <id> new [--title <title>]`
- `yai chat --ws <id> select <session_id>`
- `yai chat --ws <id> history [--session <session_id>]`
- `yai chat --ws <id> send [--session <session_id>] [--stream] <text>`

### `yai shell`
Purpose: execute capability-gated shell commands via control plane.

Usage:
- `yai shell --ws <id> exec [--cwd <path>] <cmd> [args...]`

### `yai graph`
Purpose: graph memory operations (semantic/vector/activation/awareness).

Examples:
- `yai graph add-node ...`
- `yai graph add-edge ...`
- `yai graph query ...`
- `yai graph stats ...`
- `yai graph node ...`
- `yai graph neighbors ...`
- `yai graph export ...`
- `yai graph awareness ...`
- `yai graph activate ...`
- `yai graph trace show <run_id>`

### Deprecated / Historical
- `yai tui` is removed from CLI and replaced by YX GUI.
- Historical reference remains in `law/specs/cli/TUI_COCKPIT_V1.md`.
