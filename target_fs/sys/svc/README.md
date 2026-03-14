# svc

`target_fs/sys/svc/` is the shared system-service surface.

## Scope

This domain is documentation-only at this stage.

## What stays here

- service-surface documentation only
- cross-service conventions for dispatch/registry/manifests/sockets when exposed
  as system-facing surfaces

## What does not stay here

Dispatch, manifests, registry and sockets runtime logic belong to
`target_fs/krt/svc/`.

## Boundary

`sys/svc` documents shared service-facing contracts.
`krt/svc` owns the runtime implementation.
