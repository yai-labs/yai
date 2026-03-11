# C4 Core/Runtime Final Collapse Map

Date: 2026-03-11

## Canonical decision

- `runtime` is the only canonical nucleus namespace.
- `core` is removed as active public namespace.

## Applied cutover

- removed `include/yai/core/` entirely.
- all public includes are now under `include/yai/runtime/`.
- added guardrail validator: `tools/validate/validate_runtime_core_collapse.py`.
- wired guardrail in canonical compatibility check pipeline (`tools/bin/yai-governance-compat-check`).

## Scope notes

- this pass targets namespace collapse and consumer-path guarantees.
- runtime implementation remains in `lib/runtime/` and existing governance/runtime unit+integration checks are green.
