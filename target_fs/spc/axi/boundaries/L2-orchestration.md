# L2 — System Services Plane (Orchestration Domain Focus)

## Boundary role

L2 is the `sys/` system-services plane above kernel primitives and below user interfaces.
Inside L2, `sys/orchestration/` is the orchestration execution and workflow coordination domain.

## L2 ownership model

- `kernel/` owns privileged primitives (admission, containment, grants validity, privileged registries).
- `sys/` owns service logic, lifecycle and inter-service contracts.
- `user/` owns operator/developer-facing interfaces.
- `sys/container/` is the governed execution-domain substrate consumed by orchestration.

## Orchestration domain semantics

- planning, scheduling and execution coordination
- workflow supervision across contained domains
- constrained realization of transitions and effects

## Prohibited patterns

- orchestration acting as container manager
- bypass of kernel admission/containment/grants primitives
- orchestration self-authorizing privileged effects
