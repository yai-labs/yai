---
id: ARCH-GOVERNANCE-BRIDGE
status: active
effective_date: 2026-03-08
revision: 2
owner: governance
law_refs:
  - governance/manifests/governance.manifest.json
  - governance/manifests/publish.layers.json
  - governance/runtime-package/runtime.entrypoints.json
---

# Governance Bridge

## Role

Define mandatory citation and alignment rules between `yai` runtime architecture and canonical `governance` publish/export contract.

## Rules

- Architecture docs must anchor normative claims to canonical `governance` paths or embedded contract artifacts.
- `governance/runtime-package` is the active runtime-facing contract surface.
- `../governance` is fallback bridge only and must not be used as primary reference in new docs.
- If implementation diverges from contract intent, record drift with remediation target.

## Citation pattern

Prefer:
- canonical source references (`governance/...`)
- runtime contract references (`governance/runtime-package/...`)

Avoid introducing new primary anchors under `../governance/...`.
