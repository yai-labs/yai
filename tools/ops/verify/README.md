# tools/ops/verify

Formal and baseline verification flows.

## Contents

- `core.sh`: full baseline (schema, compliance, TLC, build).
- `law-kernel.sh`: law/kernel consistency and model checking.
- `verify.sh`: wrapper dispatcher for `tools/bin/yai-verify`.

## Quick Start

- `tools/bin/yai-verify core`
- `tools/bin/yai-verify law-kernel`
