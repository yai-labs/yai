# C2 Governance/Foundation/Formal Repartition Map

Date: 2026-03-11

## Canonical boundaries

- `governance/` = policy-engine operational content (authority, grammar, registry, domains, overlays, compliance, manifests, contracts, schema, ingestion, packs, vectors).
- `foundation/` = conceptual system basis (axioms, invariants, boundaries, extensions, terminology).
- `formal/` = formal verification/modeling artifacts (TLA, formal schema, configs, traceability, artifacts).

## Relocations executed

- `governance/foundation/**` -> `foundation/**`
- `governance/formal/**` -> `formal/**`

## Duplication closure

- Removed governance-level duplicate homes for foundation and formal.
- Root now has single canonical homes for conceptual (`foundation/`) and formal (`formal/`) domains.

## Tooling cutover

- `tools/dev/check-generated.sh`: LAW_IDS target switched from `governance/formal/tla/` to `formal/tla/`.
- `tools/dev/gen-vault-abi`: split output roots (contracts under `governance/contracts`, formal IDs under `formal/tla`).
- `tools/bin/yai-governance-compat-check`: now asserts key `foundation/` and `formal/` artifacts.
- `tools/validate/validate_root_topology.py`: semantic guardrails added to prevent reintroduction of `governance/foundation` and `governance/formal`.

## Notes

- Governance runtime loaders remain canonical on `governance/` for policy-engine operational artifacts.
- Foundation/formal references in manifests and docs now map to root-level domains.
