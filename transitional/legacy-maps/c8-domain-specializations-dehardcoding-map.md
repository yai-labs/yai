# C8 — Domain Specializations Old->New Map

## Path migration
- `governance/domain-specializations/<spec>/manifest.json`
  -> `governance/domain-specializations/materialized/<spec>/manifest.json`
- `governance/domain-specializations/<spec>/(model|policy|evidence|authority|discovery|scenarios)/*`
  -> `governance/domain-specializations/materialized/<spec>/(model|policy|evidence|authority|discovery|scenarios)/*`

## Canonical semantic surfaces
- `governance/domain-specializations/index/specializations.index.json`
- `governance/domain-specializations/index/specializations.descriptors.index.json`
- `governance/domain-specializations/index/specialization.matrix.v1.json`
- `governance/domain-specializations/descriptors/*.descriptor.v1.json`

## Notes
- Materialized per-specialization bundles remain available as derived/runtime support surfaces.
- Runtime semantic resolution must read descriptors/index, not directory presence.
