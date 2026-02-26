# Docs Style Policy

This policy defines the minimum style baseline for documentation in this repository.

## Required

- Use stable IDs in frontmatter where applicable.
- Keep links repository-relative.
- Prefer deterministic language (`must`, `should`, `may`) for governance docs.
- Keep examples executable or clearly marked as pseudo examples.
- Keep section names consistent across similar artifacts.

## Prohibited

- External canonical pointers replacing local required content.
- Ambiguous closure language without PASS/FAIL criteria.
- Publishing mandatory-gate artifacts with unresolved `SKIP` checks.

## Naming

- Use kebab-case for files and directories.
- Use fixed prefixes for governance artifacts: `ADR-`, `RFC-`, `RB-`, `MP-`, `SC-`, `QT-`.

## Review

Every docs PR should include:
- impacted artifact list,
- traceability update,
- validation commands run and results.
