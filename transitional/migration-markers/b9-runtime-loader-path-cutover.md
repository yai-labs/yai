# B9 Migration Marker

Status: complete (runtime loader cutover)

## Applied

- Runtime governance loader now resolves canonical `governance/` root by default.
- Implicit primary fallback to `embedded/law` removed.
- Legacy fallback is explicit and temporary via `YAI_GOVERNANCE_ALLOW_LEGACY=1`.
- Runtime/session governance metadata lookups aligned to canonical governance root.

## Validation

- Governance unit/integration suites validate governance-first runtime behavior.
- Explicit legacy fallback behavior is covered by dedicated unit test.
