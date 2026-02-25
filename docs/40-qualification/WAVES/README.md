# WAVES Policy

Canonical source-of-truth policy for qualification outputs:

- Public/product truth: `docs/40-qualification/WAVES/<wave_id>/` bundles.
- Runtime cache/archive: `~/.yai/qualifications/...` (local, non-product artifact).

Naming policy:
- Canonical wave id must be immutable and release-linked: `WAVE-<n>-YYYY-MM-DD-<gitshortsha>`.
- Legacy wave folders without release suffix are non-canonical and must live under `_archive/`.

Operational rule:
- Keep only current canonical wave bundles in `WAVES/` root.
- Move superseded or legacy bundles to `WAVES/_archive/`.
- `WAVES/LATEST` must point to the current canonical wave directory name.
