# Documentation

Canonical documentation for the unified YAI platform.

## Navigation

- `architecture/`: active architecture by system domain.
- `guides/`: audience guides (developer, operator, user).
- `runbooks/`: executable operational procedures.
- `reference/`: technical lookup reference.
- `program/`: RFC/ADR/milestone/templates/policies/reports.
- `product/`: scenarios, demos, pre-pilot product narratives.
- `generated/`: generated documentation artifacts.
- `archive/`: historical, legacy, and migration-only material.

## Placement Rules

- Place new architecture docs only in `docs/architecture/`.
- Place user/developer/operator how-to docs in `docs/guides/`.
- Place step-by-step execution procedures in `docs/runbooks/`.
- Place lookup material in `docs/reference/`.
- Place work-governance artifacts in `docs/program/`.
- Place superseded or migration-only material in `docs/archive/`.

## Guardrails

- Do not create new top-level folders under `docs/` without updating hierarchy validation.
- Do not place migration notes in live architecture/reference/guides areas.
