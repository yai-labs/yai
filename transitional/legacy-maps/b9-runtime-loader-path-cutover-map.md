# B9 Runtime Loader Path Cutover Map

B9 cuts runtime governance loading to canonical `governance/` paths.

## Runtime Canonical Root

- `lib/governance/loader/law_loader.c`
  - governance root resolution is primary (`governance`, `YAI_GOVERNANCE_ROOT`)
  - legacy root fallback is explicit-only (`YAI_GOVERNANCE_ALLOW_LEGACY=1`)
  - canonical surface reads are default; implicit embedded fallback removed

## Loader/Resolver Canonicalization

- `lib/governance/loader/domain_model_matrix.c`
  - canonical matrix candidates only by default
  - embedded matrix fallback only when explicit legacy enabled
- `lib/governance/discovery/domain_discovery.c`
  - classification map resolves through canonical governance surface

## Runtime Consumer Alignment

- `lib/runtime/session/utils/session_utils_surface_core.inc.c`
  - governance metadata lookup is governance-first
  - embedded lookup only explicit legacy fallback

## Tests

- `tests/unit/governance/test_no_legacy_primary_path.c`
- `tests/unit/governance/test_explicit_legacy_fallback.c`
- `tests/unit/governance/run_governance_unit_tests.sh`

## Tooling Alignment

- `tools/dev/resolve-law-compat.sh`
  - governance-first compatibility root; embedded only explicit legacy mode
