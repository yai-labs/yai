# tools/bin

Canonical entrypoints for operational tooling.

## Purpose

Provide stable, easy-to-remember commands for verify, gate, suite, and diagnostics.

## Commands

- `yai-verify`: runs checks from `tools/ops/verify/`.
- `yai-gate`: runs gates from `tools/ops/gate/`.
- `yai-suite`: runs suites from `tools/ops/suite/`.
- `yai-doctor`: local environment diagnostics.
- `yai-purge`: local cleanup.

## Quick Start

- `tools/bin/yai-verify list`
- `tools/bin/yai-gate list`
- `tools/bin/yai-suite list`
