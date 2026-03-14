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
- Use canonical governance naming grammar:
  - `adr-<domain>-<nnn>-<slug>.md`
  - `rfc-<domain>-<nnn>-<slug>.md`
  - `mp-<domain>-<nnn>-<slug>(-vX-Y-Z).md`
  - runbooks and guides must use canonical terminal classes.
- Avoid fragmented micro-doc creation when an existing canonical spine can absorb the content.

## Review

Every docs PR should include:
- impacted artifact list,
- traceability update,
- validation commands run and results.
