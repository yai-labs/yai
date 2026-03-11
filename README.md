# yai

`yai` is the single canonical repository of the unified YAI system.

## Canonical roots

- runtime and headers: `cmd/`, `lib/`, `include/`
- governance spine: `governance/`
- docs authority: `docs/`
- verification/tooling: `tests/`, `tools/`

## Repository status

- split-repository topology is sunset
- `yai-law` is no longer an operational prerequisite for build/test/release in this repository
- embedded governance export surface is removed from active architecture

## Governance model

Governance is native and internal to this repository. Policy, contracts, schemas, manifests,
registry, overlays, compliance, and ingestion authoring live under `governance/` and are consumed
by runtime/tooling from canonical paths.

## Program baseline

- roadmap closure: `docs/program/repo-unico-overview-32-consegne.md`
- migration closure markers: `transitional/migration-markers/`

## Start here

- `docs/README.md`
- `governance/README.md`
- `FOUNDATION.md`
- `GOVERNANCE.md`
- `VERSIONING.md`
- `COMPATIBILITY.md`
