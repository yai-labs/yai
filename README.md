# ICE Foundation (Authoritative Spec)

This repository is the **single source of truth** for ICE specifications and documentation.
It is organized to prevent drift between canonical spec and explanatory docs.

## Hierarchy
- **core/** — axioms, invariants, terminology (authoritative)
- **formal/** — TLA+ and formal artifacts
- **boundaries/** — L0–L3 authority boundaries
- **architecture/** — system architecture by domain
- **specs/** — structured specs (non‑axiomatic)
- **protocols/** — networking and protocol documentation
- **handbook/** — guides, tutorials, development docs
- **meta/** — governance, security, licensing

## Authority Rules
- `core/` is **source of truth**.
- `handbook/` is **explanatory**.
- `architecture/` must be consistent with `core/` + `boundaries/`.

## Tooling / Site
UI themes, mkdocs/docusaurus, and site build outputs must live outside this repo or in a clearly marked `site/` tooling area.

---
Intentional. Inspectable. Deterministic.
