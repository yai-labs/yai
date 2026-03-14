# Cross-Repo Workflow

Cross-repo governance automation standards are defined in `infra`.

## Local Integration Boundaries
- This repo is integration/runtime authority.
- Compatibility and governance checks run through local gates in `tools/bin/` and `tools/release/`.

## Required Checks
- `./tools/bin/yai-governance-compat-check`
- `./tools/release/unified_repo_convergence_smoke.sh`
