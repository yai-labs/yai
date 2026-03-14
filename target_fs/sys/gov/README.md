# gov

`target_fs/sys/gov/` is the governance service surface.

## Scope

This domain keeps only the governance service entrypoint.

## What stays here

- `cmd/governanced/`: canonical governance service entrypoint
- this README as service-surface documentation

## What does not stay here

Governance loading, discovery, classification, mapping, resolution and
publication logic belong to `target_fs/krt/pol/governance/`.

## Boundary

`sys/gov` is the entry shell.
`krt/pol/governance` owns governance runtime behavior.
