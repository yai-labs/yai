# data

`target_fs/sys/data/` is the data service surface.

## Scope

This domain keeps only the service entrypoint for the data plane.

## What stays here

- `cmd/datad/`: canonical data service entrypoint
- this README as service-surface documentation

## What does not stay here

Archive flows, evidence handling, records, retention, store backends and related
internals belong to `target_fs/krt/data/`.

## Boundary

`sys/data` exposes the service shell.
`krt/data` owns the runtime implementation.
