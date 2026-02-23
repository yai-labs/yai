# Non-Core Hardening Wave 1

Tracking:
- Issue: `yai-infra#17`

## Goal
Start reducing non-core surface in `yai` while keeping CI/runtime stable.

## Wave 1 Changes
- Remove local Python test package under `tools/python/yai_tools/tests`.
- Remove stale Python cache artifacts under `tools/python/yai_tools/__pycache__`.

## Why This Is Safe
- These paths are not part of runtime build artifacts.
- Current CI workflows do not execute `tools/python/yai_tools/tests`.
- Runtime/core code (`boot/`, `root/`, `kernel/`, `engine/`, `mind/`, `runtime/`, `law/`) is untouched.

## Next Waves
- Migrate governance tooling and process docs to `yai-infra`.
- Leave only thin compatibility wrappers in `yai/tools/bin`.

## Wave 2 (Governance Docs Externalization)

- Replaced `docs/70-guides/dev-guide/*`, `docs/20-governance/templates/*`, `docs/20-governance/policies/_policy/*` with compatibility stubs.
- Canonical source moved to `yai-infra/docs/governance/yai/...`.
- `docs/60-validation/proof/*` intentionally kept in `yai` in this wave to preserve release-script compatibility.
