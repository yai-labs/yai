# Workspace Trace Ownership Model

Traces are workspace-owned and indexed at `traces/index.json`.

## Rules
- Trace refs are scoped to one workspace namespace.
- Trace summaries in inspect/debug are sourced from workspace-local state.
- Cross-workspace trace lookup is denied by workspace scope guard.
