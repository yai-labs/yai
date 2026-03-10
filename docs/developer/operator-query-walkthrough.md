# Operator Query Walkthrough (DP-8)

This walkthrough shows how operator queries read live Data Plane objects through runtime surfaces.

## Scope

Query families:
- governance
- events
- evidence
- authority
- artifacts
- graph

Runtime path:
- `cli -> sdk -> yai` (canonical)
- no direct backend query contract.

## Smoke execution

Run:

```bash
cd tests/integration/workspace_lifecycle
./workspace_operator_query_surfaces_dp8_v1.sh
```

What it validates:
1. Workspace bootstrap + governed action.
2. Query family commands return `yai.workspace.query.result.v1`.
3. Result shapes are deterministic (`table`, `timeline`, `detail_record`, `summary_card`).
4. Unsupported family is rejected (`BAD_ARGS`, `unsupported_query_family`).

## Runtime command IDs exercised

- `yai.workspace.governance.list`
- `yai.workspace.events.tail`
- `yai.workspace.evidence.list`
- `yai.workspace.authority.list`
- `yai.workspace.artifacts.list`
- `yai.workspace.graph.summary`
- `yai.workspace.query governance`

## Why this is DP and not file parsing

Returned payloads are linked to canonical runtime sinks:
- governance persistence refs
- event/evidence refs
- authority/artifact refs
- graph truth/transient refs

Operator surface reads live runtime state. Filesystem artifacts remain diagnostic/export surfaces, not primary operator contract.
