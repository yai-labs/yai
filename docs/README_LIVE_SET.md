# Live Set Statement

## Scope
This file defines the official live documentation surface after C17.8 compression.

## Official Live Families
- `docs/architecture/`
- `docs/guides/`
- `docs/runbooks/`
- `docs/reference/`
- `docs/program/`

## Canonical Entrypoints
- Architecture: `docs/architecture/README.md`
- Guides: `docs/guides/README.md`
- Runbooks: `docs/runbooks/README.md`
- Reference: `docs/reference/README.md`
- Program: `docs/program/README.md`

## Program Compression Policy
- `docs/program/milestone-packs/` is index-only live.
- `docs/program/reports/` keeps canonical core only:
  - `audit-convergence-report.md`
  - `runtime-convergence-report.md`

## Support vs Archive Policy
- Live docs must be canonical, operational, or essential reference.
- Historical, tranche, closeout, and intermediate material belongs under `docs/archive/**`.
- If a document does not justify live status, it is merged, archived, or deleted.

## Admission Rules for New Live Docs
1. A new live doc must declare a canonical owner family.
2. New report-like or milestone-like docs do not enter live by default.
3. Prefer extending existing spine docs over adding new standalone docs.
4. New live docs must pass docs naming/editorial/surface/live-set validators.
