# Datasets

Single source of truth:
- `data/datasets/`

Current canonical dataset:
- `data/datasets/global-stress/v1/`

Dataset layout contract:
- Versioning: each dataset uses semantic folder versions (`v1`, `v2`, ...).
- `seed/`: deterministic input fixtures (`*.jsonl`) for ingestion and replay.
- `prompts.csv` / workbook files: scenario and workload definitions.
- `tools/data/`: dataset-scoped operational helpers. They can read runtime state but dataset data stays under `data/datasets/`.
- `_legacy/`: deprecated or transitional datasets kept only for audit/backward checks.

Rules:
- No dataset files outside `data/datasets/`.
- `docs/` contains specs and runbooks only, never canonical data.
- Runtime artifacts do not belong in `data/datasets/` (they stay under run/artifacts roots).
