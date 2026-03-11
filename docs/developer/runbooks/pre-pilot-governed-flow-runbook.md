# Runbook: Pre-Pilot Governed Flow

## Goal

Run a complete governed workspace slice with explicit governance attachment and readable runtime outcome.

## Preconditions

- repo: `/Users/francescomaiomascio/Developer/YAI/yai`
- local toolchain available for `make`
- no stale runtime expected on `~/.yai/run/control.sock`

## Build and bootstrap

```bash
cd /Users/francescomaiomascio/Developer/YAI/yai
make -j4 yai
```

## Primary one-shot execution

```bash
tests/integration/workspace/workspace_governed_vertical_slice.sh
```

Expected terminal tail:

- `workspace_governed_vertical_slice: ok`

## Focused verification pack

```bash
tests/integration/workspace/workspace_event_surface_semantics.sh
tests/integration/workspace/workspace_flow_state_readability.sh
```

Expected:

- `workspace_event_surface_semantics_v1: ok`
- `workspace_flow_state_readability_v1: ok`

## Matrix inclusion check

```bash
rg -n "workspace_governed_vertical_slice" tests/integration/workspace/workspace_demo_matrix.sh docs/architecture/final-governed-workspace-demo-matrix.md Makefile
```

Expected:

- all three files reference `workspace_governed_vertical_slice`

## Optional operator-style flow from CLI surface

When using the CLI repo, equivalent high-level flow:

1. `yai up`
2. `yai ws create <ws>`
3. `yai ws set <ws>`
4. `yai ws policy attach customer.default.org-workspace-contextual-review`
5. `yai ws domain set --family digital --specialization remote-publication`
6. `yai ws run digital.publish sink=external_untrusted artifact=bundle-v1`
7. `yai ws run digital.publish sink=internal_trusted contract=approved destination=ops_portal artifact=bundle-v1`
8. `yai ws inspect`
9. `yai ws policy effective`
10. `yai ws debug resolution`

## Cleanup

The integration scripts perform runtime cleanup automatically.
If needed:

```bash
./build/bin/yai down
rm -f "$HOME/.yai/session/active_workspace.json"
```
