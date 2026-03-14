# con

`target_fs/sys/con/` is the container service surface.

## Scope

This domain keeps only the system-facing service shell for container management.

## What stays here

- `cmd/containerd/`: canonical service entrypoint for container management
- `image/`: bootstrap image and packaging notes
- this README as service-surface documentation

## What does not stay here

Container runtime logic, policy coupling, session binding, recovery internals,
mount handling, rootfs projection, state handling and workspace runtime behavior
belong to `target_fs/krt/con/`.

## Boundary

`sys/con` exposes and documents the service surface.
`krt/con` owns the actual runtime implementation.
