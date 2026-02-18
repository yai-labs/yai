# tools/ops/suite

Multi-step suites combining verify and gate checks.

## Structure

- `levels/`: progressive level-based suites.
- `ops/`: operational scenario suites.
- `suite.sh`: suite dispatcher wrapper.

## Quick Start

- `tools/bin/yai-suite levels/l0-l7`
- `tools/bin/yai-suite ops/no-llm-360`
