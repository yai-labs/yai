# Cross-Repo Dependency Policy

## Scope

This policy governs dependency and compatibility relations across:
- `yai`
- `yai-law`
- `yai-sdk`
- `yai-cli`

## Repository roles

- `yai-law`: normative authority source. Defines law/contracts/registries/schemas.
- `yai`: integration runtime authority. Composes runtime planes and consumes law baseline.
- `yai-sdk`: public programmatic compatibility surface consumer.
- `yai-cli`: operator compatibility surface consumer.

## Definitions

- `pinning`: structural repository dependency (submodule/vendor lock or equivalent hard bind).
- `baseline-lock`: controlled integration lock used by integration repo to guarantee reproducible behavior.
- `compatibility declaration`: explicit statement of supported law/sdk/cli surface versions without structural dependency.
- `generated/exported artifact consumption`: consumption of snapshots/manifests/exports produced by another repo, without structural pinning.

## Allowed structural relations

- `yai` -> `yai-law`: allowed (`pin` or `baseline-lock`).

## Forbidden structural relations

- `yai-law` -> any repo: forbidden.
- `yai-sdk` -> `yai-law` pinning: forbidden.
- `yai-cli` -> `yai-law` pinning: forbidden.
- `yai-cli` -> `yai-sdk` pinning: forbidden.
- Reciprocal pinning between satellite repos: forbidden.

## Compatibility model

- `yai-sdk` and `yai-cli` declare supported compatibility ranges/baselines.
- `yai-sdk` and `yai-cli` may run verify-only checks against exported/generated law surfaces.
- verify-only checks do not imply structural dependency.

## Rules going forward

1. Do not add new cross-repo pins outside `yai -> yai-law`.
2. Prefer compatibility manifests over structural linkage in satellite repos.
3. Keep generated/exported artifact workflows explicit and optional.
4. If a relation changes, update matrices in:
   - `docs/developer/cross-repo-responsibility-matrix.md`
   - `docs/developer/cross-repo-pinning-matrix.md`
