# 40 Catalog

Catalog is the capability/scenario target layer.

Catalog is not a runbook and not an implementation procedure.

Naming:
- Scenario spec: `SC-xxx`
- Qualification test gate: `QT-x.y-zzz-SCxxx`
- Evidence run: `run-00X`

Contents:
- Scenarios: `docs/40-catalog/scenarios/`
- Qualification gates: `docs/40-catalog/gates/`

Usage model:
- Catalog defines expected behavior and evidence requirements.
- Runbooks implement runtime changes needed to satisfy the catalog.
- Qualification executes deterministic pass/fail tests against catalog requirements.
