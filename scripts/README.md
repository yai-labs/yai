# Scripts

This repository keeps compatibility wrappers in `scripts/` and groups operational scripts by purpose:

- `scripts/verify/` - deterministic verification checks
- `scripts/gates/` - runtime gate checks
- `scripts/suites/` - multi-check suites
- `scripts/dev/` - local build/generation/debug helpers
- `scripts/ops/` - operator entrypoints for gate/verify/suite runners
- `scripts/data/` - dataset-related helpers

## Script Catalog

| Script | Purpose | Prerequisites | Example |
|---|---|---|---|
| `scripts/yai-verify` | Run a single verify check from `scripts/verify/` | bash, repo checkout | `scripts/yai-verify core` |
| `scripts/yai-gate` | Run a single gate check from `scripts/gates/` | bash, runtime binary as needed | `scripts/yai-gate ws dev` |
| `scripts/yai-suite` | Run a suite from `scripts/suites/` | bash, tools required by suite | `scripts/yai-suite levels/l0-l7` |
| `scripts/dev/gen-vault-abi` | Generate ABI/header artifacts from pinned specs | python3 | `scripts/dev/gen-vault-abi` |
| `scripts/dev/check-generated.sh` | Verify generated artifacts are in sync | bash, python3 | `scripts/dev/check-generated.sh` |
| `scripts/dev/yai-doctor` | Local runtime/tooling diagnostics | bash, optional `yai` in PATH | `scripts/dev/yai-doctor` |
| `scripts/ops/verify.sh` | Operator wrapper for `yai-verify` | bash | `scripts/ops/verify.sh list` |
| `scripts/ops/gate.sh` | Operator wrapper for `yai-gate` | bash | `scripts/ops/gate.sh graph graph_gate` |
| `scripts/ops/suite.sh` | Operator wrapper for `yai-suite` | bash | `scripts/ops/suite.sh ops/no-llm-360` |
| `scripts/data/fetch-embeddings.sh` | Fetch embedding artifacts for dataset workflows | curl/bash depending on script internals | `scripts/data/fetch-embeddings.sh` |
| `scripts/data/dataset-global-stress.sh` | Run dataset gate for global-stress pack | bash, runtime tooling | `scripts/data/dataset-global-stress.sh` |

Compatibility wrappers remain available in `scripts/*.sh` and delegate to canonical locations.
