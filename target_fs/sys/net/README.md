# net

`target_fs/sys/net/` is the network service surface.

## Scope

This domain keeps only the service shell for the network plane.

## What stays here

- `cmd/netd/`: canonical network service entrypoint
- optional service-facing notes for boot/integration
- this README as service-surface documentation

## What does not stay here

Discovery, routing, transport, mesh, provider runtime and topology logic belong
to `target_fs/krt/net/`.

## Boundary

`sys/net` is the service shell.
`krt/net` owns the runtime implementation.
