# Workspace Anti-Escape Baseline (WS-11)

Anti-escape baseline means the runtime prevents common scope breaks even without full OS-level sandboxing.

## Current Runtime Guards

- workspace id validation on binding resolution
- workspace scope enforcement on runtime control calls
- runtime/metadata root mismatch detection
- containment surface presence checks
- root-path traversal rejection for workspace creation inputs

## Observable Outcomes

- explicit denial reasons (for example `cross_workspace_scope_denied`)
- invalid binding surfaced as `binding_status=invalid`
- degraded containment explicitly visible in status/inspect/debug

## Next Step

Use this baseline as prerequisite for hostile-path expansion and low-level containment backends.
