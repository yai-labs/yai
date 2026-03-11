# C1 Root Canonicalization Map

Date: 2026-03-11

## Classification

A. Canonical, stays in root
- `cmd/`, `include/`, `lib/`, `governance/`, `docs/`, `tests/`, `tools/`, `data/`, `transitional/`
- Institutional files: `README.md`, `FOUNDATION.md`, `GOVERNANCE.md`, `COMPATIBILITY.md`, `VERSION`, `VERSIONING.md`, `CHANGELOG.md`
- Canonical root domains introduced in C1: `foundation/`, `formal/`

B. Relocated under canonical/transitional area
- `LAW_COMPATIBILITY.md` -> `transitional/legacy-maps/law-compatibility-root-legacy.md`
- `law-compatibility.v1.json` -> `transitional/legacy-maps/law-compatibility-root-legacy.v1.json`

C. Confined as transitional-only
- Embedded legacy marker content remains only under `transitional/embedded-law/`

D. Removed from canonical root model
- `embedded/` (removed)

## Notes

- Runtime canonical source-of-truth remains `governance/`.
- Root canonicalization guardrails are enforced by `tools/validate/validate_root_topology.py`.
