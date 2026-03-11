# C8 — Domain-Specializations De-Hardcoding Pass II

Status: implemented

## Cutover summary
- Canonical specialization model moved to descriptor/index surfaces.
- Added `domain-specializations/descriptors/` as primary semantic layer.
- Added schemas and templates for descriptor-first authoring.
- Moved per-specialization subtree bundles under `domain-specializations/materialized/`.
- Updated loaders, generators, validators, and unit coverage.

## Canonical source-of-truth
- `domain-specializations/index/specializations.index.json`
- `domain-specializations/index/specializations.descriptors.index.json`
- `domain-specializations/index/specialization.matrix.v1.json`
- `domain-specializations/descriptors/*.descriptor.v1.json`
