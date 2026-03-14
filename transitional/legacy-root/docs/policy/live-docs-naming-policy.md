---
role: support
status: active
audience: maintainer
owner_domain: docs-policy
depends_on: [docs/README.md]
---
# Live Docs Naming Policy

## Core Naming Rules
1. Filenames must be short, classificatory, and non-narrative.
2. Avoid stacked domain redundancy already implied by directory path.
3. Avoid sentence-like slugs and temporary wording.

## Forbidden Live Naming Tokens
Disallowed in live filenames:
- `closeout`
- `refoundation`
- `legacy-notes`
- `baseline`
- `wave`
- `tranche`
- `final`
- `latest`
- `temporary`
- `refactor`

## Family-Specific Rules
- ADR/RFC naming must follow the D18.2 grammar.
- Architecture/guides/reference/runbooks names must remain short and stable.
- Version-like suffixes are not default for live narrative docs.

## Examples
Valid: `architecture.md`, `workspace-isolation.md`, `audit-summary.md`.
Invalid: `workspace-verticalization-closeout-wave-final.md`, `refactor-runtime-baseline-v0-1-5.md`.
