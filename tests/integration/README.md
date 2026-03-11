# Integration Test Domains

Integration tests are grouped by operational interaction surfaces:

- `runtime/`
- `edge/`
- `mesh/`
- `orchestration/`
- `workspace/`
- `governance/`
- `source-plane/`
- `qualification/`

Boundary rule:

- `workspace/` and `qualification/` remain first-class suites.
- `source-plane/` validates owner-side ingest/read surfaces.
- `governance/` covers cross-domain resolution behavior.
