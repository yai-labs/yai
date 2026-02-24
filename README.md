# YAI — Sovereign Intelligence Runtime

YAI is the runtime implementation repository (Boot/Root/Kernel/Engine/Mind).

## Scope

This repository is runtime-first:
- implementation code
- core build and runtime checks
- runtime release artifacts

Governance/process/program documentation is externalized to `yai-infra`.

## Repository Layout

- `boot/` — machine bring-up and environment verification
- `root/` — root control plane and authority coordination
- `kernel/` — workspace isolation + policy enforcement (L1)
- `engine/` — deterministic execution and gated effects (L2)
- `mind/` — cognition/orchestration module (L3)
- `tools/` — runtime-facing wrappers and build checks
- `data/` — runtime datasets and local fixtures
- `deps/yai-specs/` — pinned normative specs (source of truth)

## Build

- `make build`
- `make dist`
- `make bundle`

Optional mind targets:
- `make mind`
- `make mind-check`

## Verification

- `make verify`
- `make release-guards`
- `make changelog-verify`

## Governance and Program Docs

Canonical location:
- `../yai-infra/docs/governance/yai/`
- `../yai-infra/docs/governance/`
- `../yai-infra/migration/`

## Security

See `SECURITY.md` and `DATA_POLICY.md`.

## License

Apache-2.0. See `LICENSE`, `NOTICE`, and `THIRD_PARTY_NOTICES.md`.
