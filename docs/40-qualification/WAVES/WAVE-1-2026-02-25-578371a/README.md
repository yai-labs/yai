---
id: SC102-WAVE1-LAUNCH
title: SC102 Wave 1 Launch Bundle
status: draft
owner: runtime
effective_date: 2026-02-25
revision: 1
---

# SC102-WAVE1-LAUNCH

SC102 star-case bundle with deterministic verify and policy pinning.

## Canonical Links
- docs/30-catalog/scenarios/SC-102.md
- docs/40-qualification/QT-0.1-003-SC102-WAVE1/README.md
- docs/30-catalog/domains/packs/D1-digital/egress-v1/README.md
- docs/30-catalog/domains/packs/D8-scientific/reproducibility-parameter-lock-v1/README.md

## Verify
```bash
./verify/verify.sh
```

## Structure
- MANIFEST.json
- INDEX.md
- POLICY/
- RUNS/deny/
- RUNS/allow/

## Release Identity
```json
{
  "yai_git_sha": "578371ae3f3fbb4240d1deb05c42086f1f561410",
  "yai_cli_git_sha": "72e487ee55de2efaa7de71374427421a923aa5ed",
  "specs_pin_sha": "20abef1874e56e4c3493df5a42697779cba00381",
  "version_label": "dev-578371a",
  "build_mode": "build",
  "reproduce": [
    "cd docs/40-qualification/QT-0.1-003-SC102-WAVE1",
    "./run/run-wave.sh",
    "cd docs/40-qualification/WAVES/SC102-WAVE1-LAUNCH",
    "./verify/verify.sh"
  ]
}
```
