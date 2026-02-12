# Datasets

Single source of truth:
- `datasets/`

Current canonical dataset:
- `datasets/global-stress/v1/`

Dataset layout contract:
- Versioning: each dataset uses semantic folder versions (`v1`, `v2`, ...).
- `seed/`: deterministic input fixtures (`*.jsonl`) for ingestion and replay.
- `prompts.csv` / workbook files: scenario and workload definitions.
- `scripts/`: dataset-scoped operational helpers. They can read runtime state but dataset data stays under `datasets/`.
- `_legacy/`: deprecated or transitional datasets kept only for audit/backward checks.

Rules:
- No dataset files outside `datasets/`.
- `docs/` contains specs and runbooks only, never canonical data.
- Runtime artifacts do not belong in `datasets/` (they stay under run/artifacts roots).
