# Governance

## Scope

This document governs the `yai` runtime implementation repository.

- Normative contracts: `deps/law/`
- Runtime implementation: `boot/`, `root/`, `kernel/`, `engine/`, `runtime/`, `mind/`

If implementation and contracts diverge, implementation must be corrected.

## Spec-First Process

For contract-facing changes:
1. Update contracts in `law` first.
2. Merge/approve contract change.
3. Align runtime implementation in `yai`.
4. Verify with CI and runtime checks.

## Documentation Residency

Governance/program docs are externalized to `infra`:
- `../infra/docs/governance/yai/`
- `../infra/docs/governance/`
- `../infra/migration/`

`yai/docs/` is runtime-first and minimal.

## Required Review Areas

- Contract pin updates and compatibility impact
- Kernel authority/enforcement boundaries
- Engine external effect gating
- Runtime protocol behavior
- Mind interaction with L1/L2 boundaries

## License

Apache-2.0.
