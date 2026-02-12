# YAI TUI Cockpit v1

> DEPRECATED: replaced by YX (YAI Experience).
> Removed from `yai` CLI as of runtime v2.5.0.
> This file is historical reference only and is not normative for current implementations.

This document described the historical behavior of `yai tui`.

## Historical Commands

- `yai tui --ws <id> run`
- `yai tui --ws <id> snapshot --view overview|graph|events|logs|db|providers|contracts|chat`

Legacy alias:

- `yai monitor --ws <id>` (deprecated, maps to `yai tui --ws <id> run`)

## Runtime Contract (Historical)

- TUI was a client of control plane and graph/db/log interfaces.
- TUI did not bypass RPC contracts for lifecycle control.
- TUI did not write runtime state directly.
- Snapshot mode was deterministic for the selected view at call time.

## Views

- `overview`
- `graph`
- `events`
- `logs`
- `db`
- `providers`
- `contracts`
- `chat`

## Notes

- `yai tui run` rendered in the current terminal session (full-screen alternate screen).
- External terminal spawn behavior in legacy `monitor` path was implementation detail.
