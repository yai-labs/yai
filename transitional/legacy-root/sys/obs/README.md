# obs

`target_fs/sys/obs/` is the observability service surface.

## Scope

This domain keeps only service entrypoints for observability-facing processes.

## What stays here

- `cmd/auditd/`: audit service entrypoint
- `cmd/metricsd/`: metrics service entrypoint
- this README as service-surface documentation

## What does not stay here

Metrics, reporting and traces runtime logic belong to `target_fs/kernel/obs/`.

## Boundary

`sys/obs` is the service shell.
`krt/obs` owns observability runtime behavior.
