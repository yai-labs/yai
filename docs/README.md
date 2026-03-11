# Documentation

Canonical documentation platform for the unified YAI repository.

## Navigation

- `architecture/`: canonical system architecture.
- `guides/`: audience-focused guides.
- `runbooks/`: executable operational procedures.
- `reference/`: technical lookup reference.
- `program/`: RFC/ADR/milestone/templates/policies/reports.
- `product/`: scenarios and product-facing narratives.
- `generated/`: generated documentation artifacts.
- `archive/`: historical, legacy, and migration-only material.

## Canonicality Rules

- Each major theme must have a single canonical source-of-truth document.
- Satellite docs may support canonical docs but must not duplicate them.
- Report/migration/closeout-style docs cannot be used as architecture source-of-truth.
- Live docs must follow naming grammar and avoid noisy historical suffixes.

## Guardrails

- Do not create new top-level docs areas without updating validation.
- Do not add migration artifacts to live architecture/guides/reference domains.
- Prefer extending canonical spine docs before adding micro-fragment documents.
