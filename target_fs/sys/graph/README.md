# graph

`target_fs/sys/graph/` is the graph service surface.

## Scope

This domain keeps only the graph service entrypoint.

## What stays here

- `cmd/graphd/`: canonical graph service entrypoint
- this README as service-surface documentation

## What does not stay here

Graph internals, lineage, materialization, query logic and summary runtime
belong to `target_fs/krt/graph/`.

## Boundary

`sys/graph` is the service shell.
`krt/graph` owns graph runtime implementation.
