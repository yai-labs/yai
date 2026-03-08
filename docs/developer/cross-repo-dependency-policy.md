# Cross-Repo Dependency Policy

## Scope

This policy governs dependency and compatibility relations across:
- `yai`
- `law`
- `sdk`
- `cli`

## Repository roles

- `law`: normative authority source. Defines law/contracts/registries/schemas.
- `yai`: integration runtime authority. Composes runtime planes and consumes law baseline.
- `sdk`: public programmatic compatibility surface consumer.
- `cli`: operator compatibility surface consumer.

## Definitions

- `pinning`: structural repository dependency (submodule/vendor lock or equivalent hard bind).
- `baseline-lock`: controlled integration lock used by integration repo to guarantee reproducible behavior.
- `compatibility declaration`: explicit statement of supported law/sdk/cli surface versions without structural dependency.
- `generated/exported artifact consumption`: consumption of snapshots/manifests/exports produced by another repo, without structural pinning.

## Allowed structural relations

- `yai` -> `law`: allowed (`pin` or `baseline-lock`).

## Forbidden structural relations

- `law` -> any repo: forbidden.
- `sdk` -> `law` pinning: forbidden.
- `cli` -> `law` pinning: forbidden.
- `cli` -> `sdk` pinning: forbidden.
- Reciprocal pinning between satellite repos: forbidden.

## Compatibility model

- `sdk` and `cli` declare supported compatibility ranges/baselines.
- `sdk` and `cli` may run verify-only checks against exported/generated law surfaces.
- verify-only checks do not imply structural dependency.

## Rules going forward

1. Do not add new cross-repo pins outside `yai -> law`.
2. Prefer compatibility manifests over structural linkage in satellite repos.
3. Keep generated/exported artifact workflows explicit and optional.
4. If a relation changes, update matrices in:
   - `docs/developer/cross-repo-responsibility-matrix.md`
   - `docs/developer/cross-repo-pinning-matrix.md`
