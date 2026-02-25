---
id: WAVE-1-2026-02-25-169b46d
title: SC102 Wave Bundle
status: draft
owner: runtime
effective_date: 2026-02-25
revision: 1
---

# WAVE-1-2026-02-25-169b46d

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
  "yai_git_sha": "169b46dfe28002107d28136563e5ceefb2ad6449",
  "yai_cli_git_sha": "ea8655d11e3567b74d4d606b65c0b2631044c987",
  "specs_pin_sha": "20abef1874e56e4c3493df5a42697779cba00381",
  "version_label": "dev-169b46d",
  "build_mode": "build",
  "reproduce": [
    "cd docs/40-qualification/QT-0.1-003-SC102-WAVE1",
    "./run/run-wave.sh",
    "cd docs/40-qualification/WAVES/<wave_id>",
    "./verify/verify.sh"
  ]
}
```
