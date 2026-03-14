# Legacy Tests (Fenced)

`tests/legacy/` is a controlled historical/regression bucket.

It is not a canonical architecture surface and does not define the current
system model.

## Canonical center

- canonical domain model: container-centered governed execution domains
- canonical ownership: `kernel/` + `sys/` + `user/`

`workspace` is explicitly legacy and transitional only.

## What legacy tests are for

- historical baseline verification
- migration regression checks
- controlled compatibility tracking during cutover

## What legacy tests are not for

- defining new primary semantics
- introducing new architecture centers
- defining the future container/orchestration model

## Hard policy

- no new primary tests may be added under `tests/legacy/workspace/`
- new canonical behavior must be tested under `tests/sys/*`, `tests/integration/*`,
  and container-centered suites
- legacy suites can be kept only with explicit migration or retirement intent
