# Testing — YAI Mind

## Test types

- Unit tests: module-level correctness
- Domain tests: per-memory-domain behaviors (`memory/graph/domains/*/tests.rs`)
- Integration tests: cross-module flows under `tests/`

## Goals

- Ensure memory graph APIs are consistent and traceable.
- Ensure RAG/session pipeline behavior remains stable.
- Ensure provider abstractions can be mocked and validated.

## Running tests

- `cargo test --all --locked`

## Determinism

Tests should be deterministic:
- avoid real network calls
- prefer mocks/fixtures
- control randomness and time where applicable
