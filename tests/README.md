# Tests

Authoritative test topology:

- `tests/unit/core`
- `tests/unit/exec`
- `tests/unit/brain`
- `tests/unit/protocol`
- `tests/unit/support`
- `tests/integration/runtime_handshake`
- `tests/integration/workspace_lifecycle`
- `tests/integration/core_exec`
- `tests/integration/core_brain`
- `tests/e2e`
- `tests/fixtures`
- `tests/shared`

Legacy taxonomy:

- `tests/domains/*` is deprecated and retained temporarily as compatibility marker only.

Primary commands:

- `make test-unit`
- `make test-integration`
- `make test-demo-matrix`
- `make test-e2e`
- `make test`
