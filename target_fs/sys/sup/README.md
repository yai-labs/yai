# sup

`target_fs/sys/sup/` is the supervisor service surface.

## Scope

This domain keeps only the supervisor service entrypoint.

## What stays here

- `cmd/supervisord/`: canonical supervisor service entrypoint
- this README as service-surface documentation

## What does not stay here

Admission, lifecycle, recovery and registry runtime logic belong to
`target_fs/krt/sup/`.

## Boundary

`sys/sup` is the service shell.
`krt/sup` owns supervisor runtime behavior.
