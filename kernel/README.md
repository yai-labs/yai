# yai-kernel

L1 Kernel implementation. This repo is runtime-critical and must remain compliant with YAI Law.
Kernel is governed by YAI Law under `../law`.
See: `../law/boundaries/L1-kernel.md` and `../law/formal/spec_map.md`.

## Authority Sources

- `../law/axioms/*`
- `../law/invariants/*`
- `../law/boundaries/*` (especially L1 Kernel boundary; L0 Vault contract; YAI Law invariants and axioms.)

## Quickstart

- `make clean && make`
- `./launcher.sh --ws arch_dev_session`

## Non-goals

No policy, intent, or authority decisions. No protocol ID changes.
