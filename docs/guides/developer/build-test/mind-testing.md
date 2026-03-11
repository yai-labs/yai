# Testing — YAI Mind (C Runtime)

## Test types

- Unit/integration tests under `tests/unit/brain/mind_legacy_tests/` (transition path)
- Runtime smoke checks through `make -C mind test`

## Goals

- Validate lifecycle init/shutdown consistency.
- Validate provider dispatch and transport request/response baseline.
- Validate memory graph and cognition pipeline baseline flows.

## Running tests

- `make -C mind test`

## Determinism

Tests should be deterministic:
- avoid external network dependencies
- use mock providers where available
- keep timing assumptions bounded
