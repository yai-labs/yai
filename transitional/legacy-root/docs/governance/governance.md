# Governance

This document defines governance operating rules for the unified YAI repository.

## Canonical governance authority

- governance content root: `governance/`
- runtime/tooling consumers: `lib/`, `tools/`, `tests/`
- canonical docs authority: `docs/`

No external governance repository is required for canonical operation.

## Operating rules

1. Update normative governance artifacts first (`governance/`).
2. Align runtime/tooling consumers to canonical paths.
3. Validate with canonical validators and smoke suites.
4. Keep migration traces only in historical docs/report areas under `docs/`.

## Boundaries

- `governance/` defines policy/contract/schema semantics.
- runtime enforces and materializes governed behavior.
- docs explain, but do not supersede normative artifacts.

## License

Apache-2.0.
