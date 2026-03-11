# C3 Law Naming Final Eradication Map

Date: 2026-03-11

## Active namespace cutover

- C symbol prefix: `yai_law_*` -> `yai_governance_*`
- Public includes: `<yai/law/*>` -> `<yai/governance/*>`
- Runtime manifest spine: `law.manifest.json` -> `governance.manifest.json`
- Runtime entrypoint field: `law_manifest_ref` -> `governance_manifest_ref`
- Manifest/schema field: `law_version` -> `governance_version`
- Formal module names:
  - `LAW_IDS` -> `GOVERNANCE_IDS`
  - `LAW_PRECEDENCE` -> `GOVERNANCE_PRECEDENCE`
  - `LAW_RESOLUTION` -> `GOVERNANCE_RESOLUTION`

## File/path cleanup

- removed `include/yai/law/` compatibility headers
- removed legacy wrappers from canonical bin:
  - `tools/bin/yai-law-sync`
  - `tools/bin/yai-law-embed-sync`
  - `tools/bin/yai-law-compat-check`
  - `tools/bin/yai-specs-sync`
- loader source renamed:
  - `lib/governance/loader/law_loader.c` -> `lib/governance/loader/governance_loader.c`

## Guardrails

- `tools/validate/validate_no_legacy_tooling_references.py` now forbids `yai-law-` token in active tooling/test roots.
- legacy `law` tokens remain only inside validator forbidden-token lists and transitional material.
