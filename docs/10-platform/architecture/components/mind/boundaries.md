# Boundaries — Mind (L3)

This document describes the boundary rules Mind must obey.

## Authority boundary

- Mind must treat every action as requiring an **explicit authority envelope**.
- Any action that could cause an external effect must be routed through governed planes (Root/Kernel/Engine) and validated.

## State boundary

- Mind may maintain internal state for reasoning, sessions, and memory graphs.
- State transitions should remain auditable and traceable (domain traces/tests exist under `memory/graph/domains/*/trace.rs`).

## Data boundary

- Default behavior must minimize data retention.
- Prompts/context fragments are treated as sensitive and must not be logged by default.
- Memory artifacts derived from real users must not be committed to git.

## Provider boundary

- Providers are untrusted inputs.
- All provider responses must be validated/sanitized before entering memory or affecting proposals.
- Provider configuration must not include secrets committed to the repo.

## Determinism boundary

- Prefer reproducible pipelines and test fixtures.
- Any non-determinism (timestamps, random seeds) should be controlled, injected, or isolated in tests.
