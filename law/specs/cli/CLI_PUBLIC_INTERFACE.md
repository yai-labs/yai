# YAI CLI Public Interface (v1)

This document defines the canonical public interface for the `yai` control suite.
It is normative. Implementations must conform.

## Scope

- Applies to the `yai` binary (Rust) and any future front-end (TUI/GUI/agents).
- The CLI is the primary interface. TUI/GUI are front-ends that invoke the same command semantics.

## Principles

1. Single Source of Truth: command semantics are defined here and in `commands.v1.json`.
2. Deterministic execution: commands must be reproducible and observable.
3. Layer boundaries remain (L1 kernel authority, L2 engine deterministic execution, L3 mind orchestration).
4. Every command supports machine output via `--json` where applicable.
5. Side effects are explicit (process spawn/kill, SHM touch, socket IO, filesystem writes).
6. Control plane is authoritative: CLI is a client and must not bypass RPC.

## Global Conventions

### Naming
- Binary: `yai`
- Subcommands: `yai <group> <action>` or `yai <action>` for top-level lifecycle.
- Flags: `--kebab-case`

### Output modes
- Default: human-readable text
- Optional: `--json` for structured output

### Exit codes
- `0` success
- `1` generic failure
- `2` invalid arguments / contract violation
- `3` dependency missing (binary not found, config missing)
- `4` runtime not ready (handshake/vault/socket not ready)

### Default Workspace
- A workspace id (`--ws`) identifies a runtime instance.
- If omitted, `ws_default` is loaded from config.

### Control Plane Paths (Canonical)
These paths are authoritative and mirror `law/specs/control/CONTROL_PLANE.md`:

- `~/.yai/run/<ws>/control.sock`
- `~/.yai/run/<ws>/lock`
- `~/.yai/run/<ws>/daemon.pid`
- `~/.yai/run/<ws>/session.json`
- `/tmp/yai_runtime.<ws>.sock`

## Command Groups

### Lifecycle
- `yai up`
- `yai down`
- `yai restart`

### Runtime Inspection
- `yai status`
- `yai ps`
- `yai logs`

### Control
- `yai providers`
- `yai sessions`

### Vault / Protocol
- `yai vault inspect`
- `yai vault dump` (optional, gated)

### Verification (Gates)
- `yai verify core`
- `yai verify law-kernel`
- `yai verify full`

### Monitor (TUI)
- `yai monitor` (full-screen)
- `yai monitor --headless` (no TUI, refresh loop)
- `yai events` (stream events)

## Command Contracts (summary)

### `yai up`
Purpose: start the runtime stack (boot + engine + mind) under a workspace id.

Required invariants:
- I-001 traceability
- I-002 determinism
- I-006 external effect boundary

Usage:
- `yai up --ws <id> [--build] [--ai] [--no-engine] [--no-mind] [--detach] [--monitor] [--timeout-ms <n>]`

Side effects:
- may spawn processes
- may create pidfiles/logfiles
- may create socket(s)

### `yai monitor`
Purpose: live cockpit. Must not mutate state unless explicitly asked.

Usage:
- `yai monitor --ws <id>`

Notes:
- TUI is a rendering of the same interface. It does not invent actions.

### `yai providers`
Purpose: discovery, trust, and attach/detach of LLM providers.

Usage:
- `yai providers discover --ws <id>`
- `yai providers list --ws <id>`
- `yai providers pair <id> <endpoint> <model> --ws <id>`
- `yai providers attach <id> [--model <m>] --ws <id>`
- `yai providers detach --ws <id>`
- `yai providers status --ws <id>`

### `yai sessions`
Purpose: list/kill active sessions.

Usage:
- `yai sessions list`
- `yai sessions kill <ws> [--force]`

(…continued in `commands.v1.json` for full detail)
