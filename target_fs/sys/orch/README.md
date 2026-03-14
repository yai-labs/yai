# orch

`target_fs/sys/orch/` is the orchestration service surface.

## Scope

This domain keeps the orchestration entry shell and high-level coordination notes.

## What stays here

- `cmd/orchestratord/`: canonical orchestration service entrypoint
- `coordination/`: service-surface coordination notes
- `scheduling/`: service-surface scheduling notes
- `supervision/`: service-surface supervision notes
- this README as service-surface documentation

## What does not stay here

Execution, workflow, planner, cognition, agent internals, bridges, transport and
runtime control belong to `target_fs/krt/orch/`.

## Boundary

`sys/orch` is the service shell.
`krt/orch` owns orchestration runtime implementation.
