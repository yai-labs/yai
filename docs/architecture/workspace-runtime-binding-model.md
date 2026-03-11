# Workspace Runtime Binding Model (6/8)

## Binding Source of Truth

Binding resolution order:

1. `YAI_ACTIVE_WORKSPACE` (explicit override)
2. cwd-mapped workspace (`~/.yai/run/*/manifest.json`, deepest `root_path` match)
3. `~/.yai/session/active_workspace.json` (compatibility fallback)

Binding status values:

- `active`
- `no_active`
- `stale`
- `invalid`

## Runtime Consumption

Runtime uses binding to resolve workspace context before control-call resolution.

Workspace commands (`current/status/inspect/domain/policy/debug/run`) consume this binding model.

In distributed acquisition v1, this binding truth remains owner-runtime (`yai`)
only; source nodes and subordinate `yai-daemon` runtimes do not become
independent binding truth.

YD-1 boundary lock:
- runtime binding is owner-side only;
- `yai-daemon` never sets canonical active workspace truth.

RF-0.2 delegation lock:
- edge-local binding/cache artifacts are delegated execution inputs only;
- they do not become workspace policy or binding truth.

RF-0.3 enforcement lock:
- local daemon binding/cache validity gates delegated enforcement eligibility;
- invalid/stale binding context cannot elevate local enforcement authority.

RF-0.4 observation lock:
- daemon can emit runtime/process observation state under owner binding context;
- observation visibility does not imply local action authority.

## Workspace-local vs Runtime-global

Workspace-local:

- declared context
- inferred/effective summaries
- trace refs
- inspect summaries

Runtime-global (current tranche):

- law embedded corpus
- runtime process primitives
- service-level ingress

Runtime-aware, workspace-scoped:

- effective resolution snapshot
- authority/evidence summaries
- effect outcome references
