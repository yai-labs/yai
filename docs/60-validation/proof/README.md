# Proof Packs

Canonical/public proof packs live under `docs/60-validation/proof/<PACK-ID>/`.

Rules:
- Draft/private packs can live in `docs/60-validation/proof/.private/` and are not tracked.
- Public packs (when promoted) must live directly under `docs/60-validation/proof/`.
- Other repos (`yai-cli`, `yai-mind`) keep pointer files only.
- Every public proof pack must pin explicit versions/tags/commits for:
  - `yai-specs`
  - `yai-cli`
  - `yai-mind`
- Every public proof pack must split:
  - existing evidence
  - missing evidence
  - non-skip gates
  - skipped gates
