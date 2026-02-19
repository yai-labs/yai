# Proof Packs (Canonical)

This directory is the single source of truth for YAI evidence/proof packs.

Rules:
- Canonical evidence lives only under `docs/proof/` in `yai`.
- Other repos (`yai-cli`, `yai-mind`) must keep pointer files only.
- Every proof pack must pin explicit versions/tags/commits for:
  - `yai-specs`
  - `yai-cli`
  - `yai-mind`
- Every proof pack must split:
  - existing evidence
  - missing evidence
  - non-skip gates
  - skipped gates

Current canonical pack:
- `docs/proof/PP-FOUNDATION-0001/`
