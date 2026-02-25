# WAVES Policy

Canonical source-of-truth policy for qualification outputs:

- Public/product truth: `docs/40-qualification/WAVES/SC102-WAVE1-LAUNCH/`
- Canonical generated build: `docs/40-qualification/WAVES/WAVE-<n>-YYYY-MM-DD-<gitshortsha>/`
- Runtime cache/archive: `~/.yai/qualifications/...` (local, non-product artifact)

Naming policy:
- Launch ID is fixed for collateral: `SC102-WAVE1-LAUNCH`
- Canonical wave id is immutable and release-linked: `WAVE-<n>-YYYY-MM-DD-<gitshortsha>`
- Legacy bundles without release suffix belong under `_archive/`

Operational rule:
- `LATEST` points to the launch alias directory name.
- Launch alias always contains: `MANIFEST.json`, `INDEX.md`, `verify.sh`, `POLICY/`, `RUNS/deny/`, `RUNS/allow/`.
- Superseded canonical builds are moved to `WAVES/_archive/` when no longer referenced.
