# Tools

This directory is the canonical home for all repository tooling.

## Layout

- `bin/`: primary operator-facing entrypoints.
- `ops/`: operational implementations for gates, suites, and verify flows.
- `dev/`: developer utilities and local diagnostics.
- `release/`: versioning, pinning, and release checks.
- `data/`: dataset-related tooling.
- `bundle/`: bundle/packaging tooling.
- `lib/`: shared shell helpers.

## Quick Start

- List verify checks: `tools/bin/yai-verify list`
- List gates: `tools/bin/yai-gate list`
- List suites: `tools/bin/yai-suite list`
