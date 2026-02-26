# 30 Catalog

Catalog is the capability/scenario target layer.

Catalog is not a runbook and not an implementation procedure.

Naming:
- Scenario spec: `SC-xxx`
- Qualification test gate: `QT-x.y-zzz-SCxxx`
- Evidence run: `run-00X`

Contents:
- Domains: `docs/30-catalog/domains/`
- Scenarios: `docs/30-catalog/scenarios/`
- Qualification gates: `docs/30-catalog/gates/`
- RealTarget trials (catalog specs): `docs/30-catalog/domains/trials/`

Usage model:
- Catalog defines expected behavior and evidence requirements.
- Runbooks implement runtime changes needed to satisfy the catalog.
- Qualification executes deterministic pass/fail tests against catalog requirements.
