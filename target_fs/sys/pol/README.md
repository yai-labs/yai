# pol

`target_fs/sys/pol/` is the policy service surface.

## Scope

This domain keeps only the policy service entrypoint.

## What stays here

- `cmd/policyd/`: canonical policy service entrypoint
- this README as service-surface documentation

## What does not stay here

Policy engine, enforcement, grants, overlays, review and state runtime belong to
`target_fs/krt/pol/`.

## Boundary

`sys/pol` is the service shell.
`krt/pol` owns policy runtime implementation.
