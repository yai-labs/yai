# WSV-8.2 Runtime Report — Workspace Bind, Persistence, and Artifact Post-Conditions

Date: 2026-03-10  
Scope: `yai` runtime-first + minimal verification updates

## Summary
WSV-8.2 post-condition hardening is implemented on the workspace lifecycle path.  
`ws create`, `ws set`, and `ws open` now enforce concrete bind/persistence post-conditions instead of returning nominal success after lightweight selection.

## Implemented changes
1. `yai_session_handle_workspace_action(... create ...)` now:
- propagates explicit failure reasons through `err`
- performs data-store workspace initialization (`yai_data_store_init_workspace`)
- reloads manifest state after write/update
- validates bound post-conditions before returning success

2. `yai_session_set_active_workspace(... set/open/switch ...)` now:
- performs data-store workspace initialization (`yai_data_store_init_workspace`)
- reloads manifest state after attachment writes
- validates bound post-conditions before returning success

3. New post-condition validator (`yai_workspace_verify_bound_postconditions`) enforces:
- workspace exists + namespace valid + containment ready
- runtime/control-plane attachment flags are true
- required containment artifacts exist (`state_surface`, `runtime_surface`, `binding_surface`)
- required store roots exist under `~/.yai/run/data/<ws>`:
  - `data`
  - `graph`
  - `knowledge`
  - `transient`
- runtime capability layer is ready and bound to the target workspace id

4. Error reporting is no longer collapsed to generic create failure:
- create-action errors now return precise reason in `exec_reply.reason`.

## Verification
Executed with real runtime UDS (outside sandbox due socket restrictions):

1. `tests/integration/workspace/workspace_runtime_reachability.sh`
- result: `PASS`
- output: `wsv81_convergence: ok (up=0 ping=0 status=0 inspect=0 graph=0)`

2. `tests/integration/workspace/workspace_binding_postconditions.sh` (new)
- result: `PASS`
- output: `wsv82_postconditions: ok (ws=... store_root=/Users/francescomaiomascio/.yai/run/data/<ws>)`

## Residual debt (non-blocking for WSV-8.2)
1. Family richness remains uneven across `ws db/data/knowledge` subcommands (some composition-backed paths remain).
2. Runtime inspect output still carries legacy-compat sections in a few areas; canonicalization cleanup remains for later WSV-8.x items.
3. CLI verbose-contract formatting cleanup is orthogonal and not closed by this runtime post-condition task.

## Disposition
WSV-8.2 objective is **met** for runtime post-conditions:
- workspace lifecycle success now implies real persistent footprint + real bound state checks
- false-positive nominal success is materially reduced
- failures emit explicit post-condition class reasons.
