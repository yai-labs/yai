# Scripts Hierarchy

Canonical script layout:

- `scripts/verify/`
  - `core.sh`
  - `law-kernel.sh`
- `scripts/gates/`
  - `ws.sh`
  - `cortex.sh`
  - `events.sh`
  - `graph.sh`
  - `providers.sh`
  - `dataset-global-stress.sh`
- `scripts/suites/levels/`
  - `l0-l7.sh`
- `scripts/suites/ops/`
  - `no-llm-360.sh`
  - `perf-slo-v1.sh`
  - `fault-injection-v1.sh`
  - `security-sanity-v1.sh`
  - `recovery-compat-v1.sh`
  - `stress-v1.sh`
- `scripts/datasets/global-stress/v1/`
  - `import-seed-via-cli.sh`
  - `load-events-log.sh`

Legacy entrypoints in `scripts/*.sh` are kept as compatibility wrappers and
delegate to this hierarchy.

Canonical runners:

- `scripts/yai-gate <name> [args...]`
- `scripts/yai-verify <name> [args...]`
- `scripts/yai-suite <path> [args...]`

Quick examples:

- `scripts/yai-gate ws dev`
- `scripts/yai-verify core`
- `scripts/yai-suite levels/l0-l7`
