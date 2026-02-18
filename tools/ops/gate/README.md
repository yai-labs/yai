# tools/ops/gate

Single-purpose operational gates.

## Main Gates

- `ws.sh`: workspace lifecycle and socket checks.
- `cortex.sh`: engine cortex event/scale checks.
- `events.sh`: event stream reliability checks.
- `graph.sh`: graph operation sanity checks.
- `providers.sh`: provider trust/selection checks.
- `providers-modes-test.sh`: strict vs non-strict provider mode checks.
- `dataset-global-stress.sh`: dataset-oriented gate.

## Quick Start

- `tools/bin/yai-gate ws <workspace>`
- `tools/bin/yai-gate providers <workspace>`
