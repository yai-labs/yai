# B13 Law Import Completeness Audit

Date: 2026-03-11
Source compared: `../law`
Destination compared: `governance/`

## File-level parity by canonical area

- authority: complete
- foundation: complete
- grammar: complete
- registry: complete
- schema: complete
- classification: complete
- domains: complete
- control-families: complete
- domain-specializations: complete
- compliance: complete
- overlays: complete
- packs: complete
- contracts: complete
- formal: complete (including TLA assets)
- vectors: complete
- ingestion: complete

## Manifest exception

One legacy export-era manifest is intentionally not canonicalized under `governance/manifests/`:

- `manifests/embedded-export.manifest.json`

It is preserved only as historical artifact at:

- `transitional/embedded-law/manifests/embedded-export.manifest.json`

Rationale: export-first embedded flow is sunset and not part of canonical 1.0.0 runtime model.
