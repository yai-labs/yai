# Workspace Shell Binding Model (6/8)

## Principle

Shell prompt token represents workspace scope from current shell cwd.

This is intentionally Git-like:

- branch indicator depends on current repository path
- workspace token depends on current workspace root path

## Semantics

- Token present only when cwd is inside a workspace root.
- Token alias comes from workspace identity alias.
- Token disappears when cwd exits workspace root.
- Token is exposed through `tools/bin/yai-ws-token` in compact form: `◉ <alias>`.

## Diagnostics

Inspect surfaces expose:

- shell cwd
- cwd relation to workspace root (`inside_workspace_root`, `outside_workspace_root`)

This prevents path/binding confusion during operations and debugging.
