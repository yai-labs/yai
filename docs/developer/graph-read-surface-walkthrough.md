# Graph Read Surface Walkthrough (DP-16)

1. Bind a workspace and run at least one governed action to materialize graph nodes/edges.
2. Query baseline graph summary:
   - `yai.workspace.graph.summary`
3. Query workspace neighborhood:
   - `yai.workspace.graph.workspace`
4. Query focused views:
   - `yai.workspace.graph.governance`
   - `yai.workspace.graph.decision`
   - `yai.workspace.graph.artifact`
   - `yai.workspace.graph.authority`
   - `yai.workspace.graph.evidence`
5. Query lineage and recent classes:
   - `yai.workspace.graph.lineage`
   - `yai.workspace.graph.recent`

Expected behavior:
- All responses are runtime-mediated query objects (`yai.workspace.query.result.v1`).
- Read path reports `mode=db_first` and `filesystem_primary=false`.
- Graph truth is authoritative; transient cognition is not promoted to truth.
