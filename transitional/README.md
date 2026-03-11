# Transitional Migration Spine

`transitional/` is a non-canonical migration archive.

Purpose:

- preserve migration evidence and crosswalks
- keep historical cutover markers auditable
- avoid contaminating canonical runtime/governance/docs/tooling roots

Canonical roots remain:

- `governance/`
- `docs/`
- `cmd/`, `lib/`, `include/`
- `tests/`, `tools/`

## Status

- not part of canonical 1.0.0 runtime/governance model
- shrink-to-zero policy remains active after convergence

## Structure

- `embedded-law/`: historical embedded shutdown traces
- `legacy-docs/`: archived doc references
- `legacy-maps/`: old-to-new path maps
- `migration-markers/`: tranche closure markers
