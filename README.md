# yai

`yai` is the single canonical repository of the unified YAI system.

## Canonical roots

- runtime and headers: `cmd/`, `lib/`, `include/`
- governance spine: `governance/`
- platform foundation domain: `foundation/`
- platform formal domain: `formal/`
- docs authority: `docs/`
- verification/tooling: `tests/`, `tools/`

## Repository status

- split-repository topology is sunset
- the former separate governance repository is no longer an operational prerequisite for build/test/release in this repository
- embedded legacy surfaces are removed from active root topology (historical traces are transitional-only)

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
