---
id: WAVE-1-2026-02-25-bf59d51
title: SC102 Wave Bundle
status: draft
owner: runtime
effective_date: 2026-02-25
revision: 1
---

# WAVE-1-2026-02-25-bf59d51

SC102 wave bundle with selected evidence runs and deterministic verify.

## Canonical Links
- docs/30-catalog/scenarios/SC-102.md
- docs/40-qualification/QT-0.1-003-SC102-WAVE1/README.md
- docs/30-catalog/domains/packs/D1-digital/egress-v1/README.md
- docs/30-catalog/domains/packs/D8-scientific/reproducibility-parameter-lock-v1/README.md
- docs/30-catalog/domains/trials/D1-digital/egress-v1/RT-001-curl-egress-v1/README.md
- docs/30-catalog/domains/trials/D1-digital/egress-v1/RT-002-otel-export-egress-v1/README.md
- docs/30-catalog/domains/trials/D1-digital/egress-v1/RT-003-s3-upload-egress-v1/README.md
- docs/30-catalog/domains/trials/D8-scientific/reproducibility-parameter-lock-v1/RT-001-params-lock-v1/README.md

## Verify
```bash
./verify/verify.sh
```

## Release Identity
```json
{
  "yai_git_sha": "bf59d51941456042b71f268468d9a62dbf53d74d",
  "yai_cli_git_sha": "72e487ee55de2efaa7de71374427421a923aa5ed",
  "specs_pin_sha": "20abef1874e56e4c3493df5a42697779cba00381",
  "version_label": "dev-bf59d51",
  "build_mode": "build",
  "reproduce": [
    "cd docs/40-qualification/QT-0.1-003-SC102-WAVE1",
    "./run/run-wave.sh",
    "cd docs/40-qualification/WAVES/<wave_id>",
    "./verify/verify.sh"
  ]
}
```
