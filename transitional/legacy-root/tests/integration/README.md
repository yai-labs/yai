# Integration Test Domains

Integration tests are grouped by operational interaction surfaces:

- `container/`
- `sys/*` service integrations
- transitional historical suites moved under `tests/legacy/*`

Boundary rule:

- `workspace` is legacy-only and not a canonical integration center.
- source-plane scripts that are workspace-centered are fenced under `tests/legacy/source-plane/`.
- canonical integration growth must target container/system service planes.
